use std::{
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

/// Returns `stacks`, a num_stacks long vector of variable length vectors
/// holding the chars corresponding to crates on each stack, and a lineiter
/// which can be used to iterate over the 'instructio;ns'
fn parse_input(filename: &str) -> (Vec<Vec<char>>, io::Lines<io::BufReader<File>>) {
    // Parsing the input for this one seems tricky.
    // Luckily the input -does- have some regularity. All the entries in each
    // column are three characters followed by a space. We can use this to parse
    let file = File::open(filename).unwrap();
    let mut linebuf: Vec<String> = vec![];
    let filebuf = io::BufReader::new(file);
    let mut lineiter = filebuf.lines();
    while let Some(line) = lineiter.next() {
        let line = line.unwrap();
        if line == "" {
            break;
        }
        linebuf.push(line);
    }
    // Now we have all the crate lines in a buffer, we iterate in reverse order
    // and populate stacks with them
    let mut cratelines = linebuf.iter().rev();
    let numstack = (cratelines.next().expect("Bad input").len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; numstack];
    for line in cratelines {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    (stacks, lineiter)
}

fn print_tops(stacks: &Vec<Vec<char>>) -> () {
    println!("{}", stacks.iter().map(|x| x.last().unwrap()).collect::<String>());
}

fn cratemover_9000(stacks: &mut Vec<Vec<char>>, instruction: String) {
    let rx = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    if let Some(c) = rx.captures(&instruction[..]) {
        let num_to_move = &c[1].parse::<usize>().unwrap();
        let stack_from = &c[2].parse::<usize>().unwrap();
        let stack_to = &c[3].parse::<usize>().unwrap();
        for _ in 0..*num_to_move {
            let val = stacks[*stack_from-1].pop().unwrap();
            stacks[*stack_to-1].push(val);
        }

    }
}

fn cratemover_9001(stacks: &mut Vec<Vec<char>>, instruction: String) {
    let rx = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    if let Some(c) = rx.captures(&instruction[..]) {
        let num_to_move = &c[1].parse::<usize>().unwrap();
        let stack_from = &c[2].parse::<usize>().unwrap();
        let stack_to = &c[3].parse::<usize>().unwrap();
        
        let split_idx = stacks[*stack_from-1].len() - *num_to_move;
        let move_crates = stacks[*stack_from-1].split_off(split_idx);
        stacks[*stack_to-1].extend(move_crates);
    }
}


fn main() {
    // Part 1
    let (mut stacks, mut lineiter) = parse_input("day5_1.txt");
    
    while let Some(line) = lineiter.next() {
        let line = line.unwrap();
        cratemover_9000(&mut stacks, line);
    }
    print_tops(&stacks);
    // Part 2
    let (mut stacks, mut lineiter) = parse_input("day5_1.txt");
    
    while let Some(line) = lineiter.next() {
        let line = line.unwrap();
        cratemover_9001(&mut stacks, line);
    }
    print_tops(&stacks);
}
