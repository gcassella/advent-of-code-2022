use std::{
    fs::File,
    io::{self, BufRead},
};

enum Command {
    Noop,
    Addx(i32),
}

// Increment the cycle counter. If we're at an output cycle return a value.
fn advance_cycle(cycle: &mut usize, register: &i32) -> Option<i32> {
    if (register - (*cycle as i32 % 40)).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }
    if *cycle > 0 && (*cycle + 1) % 40 == 0 {
        print!("\n");
    }
    *cycle += 1;
    if (*cycle >= 20) && ((*cycle - 20) % 40) == 0 {
        return Some(*register);
    }
    None
}

fn main() {
    // Init input reader
    let file = File::open("day10_1.txt").unwrap();
    let filebuf = io::BufReader::new(file);
    let lineiter = filebuf.lines();
    // Read input into a command buffer
    let mut commands: Vec<Command> = vec![];
    for line in lineiter {
        let line = line.unwrap();
        let input = line.split(' ').collect::<Vec<&str>>();
        let cmd = match input[0] {
            "noop" => Command::Noop,
            "addx" => Command::Addx(input[1].parse::<i32>().unwrap()),
            _ => panic!("Malformed input"),
        };
        commands.push(cmd);
    }
    // Loop over commands and update buffer, printing at every
    // (cycle-20)%40 == 0 cycles
    let mut cycle: usize = 0;
    let mut register: i32 = 1;
    let mut seen: Vec<i32> = vec![];
    for cmd in commands.iter() {
        match cmd {
            Command::Noop => {
                // mid cycle 1

                match advance_cycle(&mut cycle, &register) {
                    Some(reg) => seen.push(cycle as i32 * reg),
                    None => (),
                }
                // end cycle 1
            }
            Command::Addx(val) => {
                // mid cycle 1
                match advance_cycle(&mut cycle, &register) {
                    Some(reg) => seen.push(cycle as i32 * reg),
                    None => (),
                }
                // mid cycle 2
                match advance_cycle(&mut cycle, &register) {
                    Some(reg) => seen.push(cycle as i32 * reg),
                    None => (),
                }
                // end cycle 2
                register += val;
            }
        }
    }
    print!("\n");
    println!("{:?}", seen);
    println!("{:?}", seen.iter().fold(0, |a, b| a + b));
}
