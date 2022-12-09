use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    distance: usize,
}

fn dndmetric(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
    let xdiff = (p1.0 - p2.0).abs();
    let ydiff = (p1.1 - p2.1).abs();
    if xdiff > ydiff {
        xdiff
    } else {
        ydiff
    }
}

fn update_tail(tail_pos: &(i32, i32), head_pos: &(i32, i32)) -> (i32, i32) {
    let diff = (
        (head_pos.0 - tail_pos.0).clamp(-1, 1),
        (head_pos.1 - tail_pos.1).clamp(-1, 1),
    );
    (tail_pos.0 + diff.0, tail_pos.1 + diff.1)
}

fn main() {
    // Init input reader
    let file = File::open("day9_1.txt").unwrap();
    let filebuf = io::BufReader::new(file);
    let lineiter = filebuf.lines();
    // Read input into a command buffer
    let mut commands: Vec<Command> = vec![];
    for line in lineiter {
        let line = line.unwrap();
        let input = line.split(' ').collect::<Vec<&str>>();
        let cmd = Command {
            direction: match input[0] {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!(),
            },
            distance: input[1].parse().unwrap(),
        };
        commands.push(cmd);
    }

    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);
    let mut visited_positions: Vec<(i32, i32)> = vec![(0, 0)];

    for cmd in commands {
        for _ in 0..cmd.distance {
            match cmd.direction {
                Direction::Up => {
                    head_pos.1 += 1 as i32;
                }
                Direction::Right => {
                    head_pos.0 += 1 as i32;
                }
                Direction::Down => {
                    head_pos.1 -= 1 as i32;
                }
                Direction::Left => {
                    head_pos.0 -= 1 as i32;
                }
            }

            if dndmetric(&head_pos, &tail_pos) > 1 {
                tail_pos = update_tail(&tail_pos, &head_pos);
                if !visited_positions.contains(&tail_pos) {
                    visited_positions.push(tail_pos.clone());
                }
            };
        }
    }

    println!("{}", visited_positions.len());
}
