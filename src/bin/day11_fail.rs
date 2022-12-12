use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    ops,
    f64
};

static BIGGY_BASE: usize = 1000;

#[derive(Debug)]
struct BigInt {
    digits: Vec<usize>,
    base: usize
}

impl BigInt {
    fn from(num: usize, base: usize) -> BigInt {
        let num_digits: usize = (num as f64).log(base as f64).ceil() as usize;
        let mut remainder = num;
        let mut digits: Vec<usize> = vec![];
        for i in (0..num_digits).rev() {
            let digit = ((remainder as f64) / (base.pow(i as u32) as f64)).floor() as usize;
            digits.push(digit);
            remainder -= digit * base.pow(i as u32);
        }
        
        BigInt { digits: digits.iter().rev().map(|x| *x).collect(), base }
    }

    fn is_zero(self) -> bool {
        for digit in self.digits {
            if digit != 0 {
                return false;
            }
        }
        true
    }
}

impl ops::Add<BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, _rhs: BigInt) -> BigInt {
        assert!(self.base == _rhs.base);
        let n = self.digits.len();
        let m = _rhs.digits.len();

        let num_digits = if n > m { n } else { m };

        let mut carry = 0;
        let mut digits: Vec<usize> = vec![];
        for i in 0..num_digits {
            // we use the isize cast here because n/m - 1 can be zero
            let lhs = if i as isize > n as isize - 1 {
                0
            } else { self.digits[i] };
            let rhs = if i as isize > m as isize - 1 {
                0
            } else { _rhs.digits[i] };
            let d = lhs + rhs + carry;
            let digit = d % self.base;
            carry = d / self.base;
            digits.push(digit);
        }
        if carry != 0 {
            digits.push(1);
        }
        BigInt { digits, base: self.base }
    }
}

impl ops::Mul<BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, _rhs: BigInt) -> BigInt {
        assert!(self.base == _rhs.base);
        let num_digits: usize = self.digits.len() + _rhs.digits.len();

        let mut digits: Vec<usize> = vec![0; num_digits];

        for (i, digit_a) in self.digits.iter().enumerate() {
            for (j, digit_b) in _rhs.digits.iter().enumerate() {
                let d = digit_a * digit_b;
                digits[i + j] += d % self.base;
                // handle possible carry
                digits[i + j + 1] += d / self.base;
            }
        }
        // check if we need to propagate carry
        for m in 0..num_digits {
            if digits[m] >= self.base {
                let digit = digits[m];
                digits[m] = digit%self.base;
                digits[m+1] += digit/self.base;
            }
        }
        let last_digit = digits[digits.len()-1].to_owned();
        // might not need highest order digit
        if last_digit == 0 {
            digits.pop();
        }
        // might need to carry highest order digit
        if last_digit > self.base {
            let end = digits.len()-1;
            digits[end] = last_digit%self.base;
            digits.push(last_digit/self.base);
        }
        BigInt { digits, base: self.base }
    }
}

impl ops::Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(self, _rhs: BigInt) -> BigInt {
        // FIXME: doesn't account for lhs < rhs. don't care for this problem.
        assert!(self.base == _rhs.base);

        let n = self.digits.len();
        let m = _rhs.digits.len();

        let max = if m > n { m } else { n };
        let mut carry = self.base;

        let mut digits: Vec<usize> = vec![];

        for i in 0..max {
            let other_digit = if i > _rhs.digits.len()-1 { 0 } else { _rhs.digits[i] };
            carry = self.base-1+self.digits[i]-other_digit+carry/self.base;
            digits.push(carry % self.base);
        }

        if digits.last().unwrap() == &0 {
            digits.pop();
        }

        BigInt { digits, base: self.base }
    }
}

impl PartialEq<BigInt> for BigInt {
    fn eq(&self, other: &BigInt) -> bool {
        self.digits == other.digits && self.base == other.base
    }
}

impl PartialOrd<BigInt> for BigInt {
    fn partial_cmp(&self, other: &BigInt) -> Option<std::cmp::Ordering> {
        assert!(self.base == other.base);
        if self.digits.len() > other.digits.len() {
            return Some(std::cmp::Ordering::Greater);
        } else if self.digits.len() < other.digits.len() {
            return Some(std::cmp::Ordering::Less);
        }
        for i in (0..self.digits.len()).rev() {
            if self.digits[i] > other.digits[i] {
                return Some(std::cmp::Ordering::Greater);
            } else if self.digits[i] < other.digits[i] {
                return Some(std::cmp::Ordering::Less);
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }
}

impl ops::Rem<BigInt> for BigInt {
    type Output = BigInt;

    fn rem(self, _rhs: BigInt) -> BigInt {
        let n = self.digits.len();
        let m = _rhs.digits.len();
        let mut remainder = self.clone();
        let mut divisor: usize = 0;
        let mut i: usize = 0;

        while remainder >= _rhs.clone() {
            let sub_operand = if n - m > i {
                divisor += self.base.pow((n - m - i) as u32);
                _rhs.clone()*BigInt::from(self.base.pow((n - m - i) as u32), self.base)
            } else {
                divisor += 1;
                _rhs.clone()
            };

            remainder = remainder - sub_operand;
            i += 1;
        }

        remainder
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        Self { digits: self.digits.clone(), base: self.base }
    }
}

enum OpInput {
    Num(usize),
    Old,
}

enum Op {
    Add,
    Multiply,
}

struct Monkey {
    id: usize,
    items: VecDeque<BigInt>,
    op: Box<dyn Fn(BigInt) -> BigInt>,
    test_divisor: usize,
    true_target: usize,
    false_target: usize,
    inspections: usize,
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey {} with items {:?} and test/targets {} {} {}",
            self.id, self.items, self.test_divisor, self.true_target, self.false_target
        )
    }
}

fn parse_op(opstring: &String) -> Box<dyn Fn(BigInt) -> BigInt> {
    let terms = opstring.split(" ");
    let mut operands: Vec<OpInput> = vec![];
    let mut operation = Op::Add;
    for term in terms {
        match term {
            "+" => {
                operation = Op::Add;
            }
            "*" => {
                operation = Op::Multiply;
            }
            _ => match term.parse::<usize>() {
                Ok(val) => operands.push(OpInput::Num(val)),
                Err(_) => operands.push(OpInput::Old),
            },
        };
    }

    Box::new(move |old| {
        let a = match operands[0] {
            OpInput::Num(val) => BigInt::from(val, BIGGY_BASE),
            OpInput::Old => old.clone(),
        };
        let b = match operands[1] {
            OpInput::Num(val) => BigInt::from(val, BIGGY_BASE),
            OpInput::Old => old.clone(),
        };
        match operation {
            Op::Add => return a + b,
            Op::Multiply => return a * b,
        }
    })
}

impl Monkey {
    fn from_lines(lines: Vec<&String>) -> Monkey {
        lazy_static! {
            // regex for capturing a simple usize, i.e. id, divisor, targets
            static ref NUM_RE: Regex = Regex::new(r"(\d+)").unwrap();
            // regex for capturing list of usize, i.e. starting items
            static ref ITEMS_RE: Regex = Regex::new(r"Starting items: ((?:\d+(?:, )?)*)").unwrap();
            // regex for capturing operation
            static ref OP_RE: Regex = Regex::new(r"new = (.*)").unwrap();
        }
        // Check input came to us correctly
        assert!(lines[0].starts_with("Monkey"));
        assert!(lines.len() == 6);
        // Parse lines
        let id: usize = NUM_RE
            .captures(&lines[0][..])
            .unwrap()
            .get(1)
            .map(|x| x.as_str().parse().unwrap())
            .unwrap();
        let items = VecDeque::from_iter(
            ITEMS_RE
                .captures(&lines[1][..])
                .unwrap()
                .get(1)
                .map(|x| x.as_str())
                .unwrap()
                .split(", ")
                .map(|x| BigInt::from(x.parse::<usize>().unwrap(), BIGGY_BASE)),
        );
        let op = parse_op(&String::from(
            OP_RE
                .captures(&lines[2][..])
                .unwrap()
                .get(1)
                .map(|x| x.as_str())
                .unwrap(),
        ));
        let test_divisor: usize = NUM_RE
            .captures(&lines[3][..])
            .unwrap()
            .get(1)
            .map(|x| x.as_str().parse().unwrap())
            .unwrap();
        let true_target: usize = NUM_RE
            .captures(&lines[4][..])
            .unwrap()
            .get(1)
            .map(|x| x.as_str().parse().unwrap())
            .unwrap();
        let false_target: usize = NUM_RE
            .captures(&lines[5][..])
            .unwrap()
            .get(1)
            .map(|x| x.as_str().parse().unwrap())
            .unwrap();

        Monkey {
            id,
            items,
            op: Box::new(op),
            test_divisor,
            true_target,
            false_target,
            inspections: 0,
        }
    }
}

/// This solution DOESNT WORK!!! no matter how you slice it, trying to solve
/// this brute force with an arbitrary precision arithmetic implementation
/// will always fail. the difference in magnitudes of the operators just gets
/// TOO DAMN BIG. I will have to be clevererer.
fn main() {
    // Init input reader
    let file = File::open("day11_1.txt").unwrap();
    let filebuf = io::BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();
    // Read input
    let mut monkeys: Vec<Monkey> = vec![];
    for i in 0..(all_lines.len() + 1) / 7 {
        let lines: Vec<&String> = all_lines[i * 7..i * 7 + 6].iter().collect();
        let monkey = Monkey::from_lines(lines);
        monkeys.push(monkey);
    }
    let nrounds = 20;
    // Execute monkey loop
    for round in 0..nrounds {
        println!("BEGINNING ROUND {}", round);
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            println!("{}", monkey.id);
            let mut item_buffer: Vec<BigInt> = vec![];
            let mut target_buffer: Vec<usize> = vec![];
            while monkey.items.len() > 0 {
                let mut item = monkey.items.pop_front().unwrap();
                println!("  Monkey inspects an item with a worry level of {:?}.", item.clone());
                monkey.inspections += 1;
                println!("    Worry level becomes {:?}", (monkey.op)(item.clone()));
                item = (monkey.op)(item.clone());
                println!("    Worry level is divided by 3 to {:?}", item.clone());
                item_buffer.push(item.clone());
                let target = if (item.clone() % BigInt::from(monkey.test_divisor, BIGGY_BASE)).is_zero() {
                    println!(
                        "    Current worry level is divisible by {}.",
                        monkey.test_divisor
                    );
                    println!(
                        "    Item with worry level {:?} is thrown to monkey {}.",
                        item.clone(), monkey.true_target
                    );
                    monkey.true_target
                } else {
                    println!(
                        "    Current worry level is not divisible by {}.",
                        monkey.test_divisor
                    );
                    println!(
                        "    Item with worry level {:?} is thrown to monkey {}.",
                        item, monkey.false_target
                    );
                    monkey.false_target
                };
                target_buffer.push(target);
            }
            drop(monkey);
            // throw out items
            for (item, target) in item_buffer.iter().zip(target_buffer) {
                monkeys[target].items.push_back(item.clone());
            }
        }
    }

    for monkey in monkeys.iter() {
        println!(
            "Monkey {} inspected items {} times.",
            monkey.id, monkey.inspections
        );
    }
}