use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
};

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
    items: VecDeque<usize>,
    op: Box<dyn Fn(usize) -> usize>,
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

fn parse_op(opstring: &String) -> Box<dyn Fn(usize) -> usize> {
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
            OpInput::Num(val) => val,
            OpInput::Old => old,
        };
        let b = match operands[1] {
            OpInput::Num(val) => val,
            OpInput::Old => old,
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
                .map(|x| x.parse::<usize>().unwrap()),
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
    // let nrounds = 20; // uncomment me for part 1
    let nrounds = 10000;
    // Find lowest monkey denominator
    let divisors: Vec<usize> = monkeys.iter().map(|x| x.test_divisor).collect();
    let lcm = divisors.iter().fold(1, |a, b| a * b);
    // Execute monkey loop
    for _ in 0..nrounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            println!("{}", monkey.id);
            let mut item_buffer: Vec<usize> = vec![];
            let mut target_buffer: Vec<usize> = vec![];
            while monkey.items.len() > 0 {
                let mut item = monkey.items.pop_front().unwrap();
                println!("  Monkey inspects an item with a worry level of {}.", item);
                monkey.inspections += 1;
                println!("    Worry level becomes {}", (monkey.op)(item));
                // item = (monkey.op)(item) / 3; // uncomment me for part 1
                item = (monkey.op)(item) % lcm;
                println!("    Worry level is divided by 3 to {}", item);
                item_buffer.push(item);
                let target = if item % monkey.test_divisor == 0 {
                    println!(
                        "    Current worry level is divisible by {}.",
                        monkey.test_divisor
                    );
                    println!(
                        "    Item with worry level {} is thrown to monkey {}.",
                        item, monkey.true_target
                    );
                    monkey.true_target
                } else {
                    println!(
                        "    Current worry level is not divisible by {}.",
                        monkey.test_divisor
                    );
                    println!(
                        "    Item with worry level {} is thrown to monkey {}.",
                        item, monkey.false_target
                    );
                    monkey.false_target
                };
                target_buffer.push(target);
            }
            drop(monkey);
            // throw out items
            for (item, target) in item_buffer.iter().zip(target_buffer) {
                monkeys[target].items.push_back(*item);
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
