use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

//  = -> -2
//  - -> -1
//  0 -> 0
//  1 -> 1
//  2 -> 2
//
// can just construct new digits in base 5
// 0 -> 0
// 1 -> 1
// 2 -> 2
// 3 -> 1=
// 4 -> 1-
// 5 -> 10
// 6 -> 11
// 7 -> 12
// 8 -> 2=
#[derive(Debug, Clone, PartialEq, Eq)]
struct SNAFU {
    digits: Vec<char>,
}

impl SNAFU {
    fn from(num: usize) -> SNAFU {
        // convert to base 5

        let num_digits: usize = (num as f64).log(5.0).floor() as usize + 1;
        let mut remainder = num;
        let mut digits: Vec<usize> = vec![];
        for i in (0..num_digits).rev() {
            let digit = ((remainder as f64) / (5usize.pow(i as u32) as f64)).floor() as usize;
            digits.push(digit);
            remainder -= digit * 5usize.pow(i as u32);
        }
        // convert base 5 to snafu
        let mut carry: usize = 0;
        let mut SNAFU_digits: Vec<char> = vec![];
        for (i, digit) in digits.iter_mut().enumerate().rev() {
            *digit += carry;
            if *digit > 2 {
                // roll over
                carry = 1;
                match digit {
                    3 => SNAFU_digits.push('='),
                    4 => SNAFU_digits.push('-'),
                    5 => SNAFU_digits.push('0'),
                    _ => unreachable!(),
                };
            } else {
                carry = 0;
                SNAFU_digits.push(char::from_digit(*digit as u32, 5).unwrap());
            }
        }
        if carry == 1 {
            SNAFU_digits.push('1');
        }
        SNAFU_digits.reverse();
        SNAFU {
            digits: SNAFU_digits,
        }
    }

    fn to_decimal(&self) -> usize {
        let mut result: usize = 0;
        let n = self.digits.len() - 1;
        for (i, digit) in self.digits.iter().enumerate() {
            let mul = match digit {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            };
            let bval = 5i64.pow((n - i) as u32);
            result = (result as i64 + mul * bval) as usize;
        }
        result
    }
}

fn main() {
    // Init input reader
    let file = File::open(r"./src/data/day25_1.txt").unwrap();
    let filebuf = BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();
    //
    let mut total = 0;
    for line in all_lines {
        total += (SNAFU {
            digits: line.chars().collect::<Vec<char>>(),
        })
        .to_decimal();
    }
    println!("{:?}", SNAFU::from(total).digits.iter().collect::<String>());
}
