use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// You have to move the data into the same folder as the binary yourself.
/// Sorry.
fn main() {
    let f = File::open("day1_1.txt").expect("File not found");

    let calories = read_input(f);
    let maxcal = find_max(&calories);
    println!("Max calories: {}", maxcal);

    let topthreecal = find_top_three(&calories);
    println!(
        "Sum of top three calories: {}",
        topthreecal.iter().sum::<usize>()
    );
}

fn read_input(f: File) -> Vec<usize> {
    let mut calories: Vec<usize> = vec![];
    let mut calbuf: usize = 0;
    for line in BufReader::new(f).lines() {
        let cal = line.unwrap().parse::<usize>();
        match cal {
            Ok(c) => {
                calbuf += c;
            }
            Err(_) => {
                calories.push(calbuf);
                calbuf = 0;
            }
        }
    }
    calories.push(calbuf);
    calories
}

fn find_max(calories: &Vec<usize>) -> usize {
    let mut max = 0;

    for cal in calories {
        if *cal > max {
            max = *cal;
        }
    }
    max
}

fn find_top_three(calories: &Vec<usize>) -> Vec<usize> {
    let mut top_three_vals = vec![0, 0, 0];
    let mut smallest_top_three = 0;

    for cal in calories {
        if *cal > smallest_top_three {
            top_three_vals.push(*cal);
            top_three_vals.sort_by(|a, b| b.cmp(a));
            top_three_vals.pop();
            smallest_top_three = top_three_vals[2];
        }
    }
    top_three_vals
}
