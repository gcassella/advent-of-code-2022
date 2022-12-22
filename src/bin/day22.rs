use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(Direction),
}

struct MapRow {
    tiles: Vec<usize>,
    left_idx: usize,
    right_idx: usize,
}

/// Return the next instruction in instr_str and remove it from the string
fn next_instruction(instr_str: &mut String) -> Option<Instruction> {
    let mut peekable_itr = instr_str.chars().peekable();
    let mut num_digits: usize = 0;
    let mut steps: usize = 0;
    loop {
        let char = peekable_itr.next();
        let next_char = peekable_itr.peek();
        match char {
            Some(val) => match val {
                'R' => {
                    instr_str.remove(0);
                    return Some(Instruction::Turn(Direction::Right));
                }
                'L' => {
                    instr_str.remove(0);
                    return Some(Instruction::Turn(Direction::Left));
                }
                _ => {
                    num_digits += 1;
                    steps = steps * 10 + val.to_digit(10).unwrap() as usize;
                }
            },
            None => return None,
        }
        // keep accumulating digits until we hit L, R or line end
        match next_char {
            Some(val) => match val {
                'R' => break,
                'L' => break,
                _ => continue,
            },
            None => break,
        }
    }
    for _ in 0..num_digits {
        instr_str.remove(0);
    }
    Some(Instruction::Move(steps))
}

fn main() {
    // Init input reader
    let file = File::open(r"./src/data/day22_1.txt").unwrap();
    let filebuf = BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();
    //
    let mut lineiter = all_lines.iter();
    let mut map: Vec<MapRow> = vec![]; // map[row][column] = 1 (0) if blocked (open)
    for line in lineiter {
        if line.is_empty() {
            break;
        }
        let mut left_idx = 0;
        let mut right_idx = 0;
        let mut started = false;
        let mut tiles: Vec<usize> = vec![];
        for char in line.chars() {
            if char == ' ' {
                if !started {
                    left_idx += 1;
                } else {
                    break;
                }
            } else if char == '.' {
                tiles.push(0);
            } else if char == '#' {
                tiles.push(1);
            }

            right_idx += 1;
        }
        map.push(MapRow {
            tiles,
            left_idx,
            right_idx,
        });
    }
    let mut instructions = all_lines.last().unwrap().clone();
    //
    let mut curr_pos: (usize, usize) = (0, 0);
    let mut curr_fac: (isize, isize) = (0, 1);

    let mut positions: Vec<(usize, usize)> = vec![];
    let mut facings: Vec<(isize, isize)> = vec![];

    while instructions.len() > 0 {
        let instruction = next_instruction(&mut instructions).unwrap();
        match instruction {
            // These conversions can underflow given a malformed input.
            Instruction::Move(steps) => {
                for _ in 0..steps {
                    positions.push((curr_pos.0, curr_pos.1 + map[curr_pos.0].left_idx));
                    facings.push(curr_fac);
                    curr_pos = take_step(&map, curr_pos, curr_fac);
                }
            }
            Instruction::Turn(dir) => {
                match dir {
                    Direction::Left => {
                        // (0, 1) -> (-1, 0)
                        // (-1, 0) -> (0, -1)
                        // (0, -1) -> (1, 0)
                        // (1, 0) -> (0, 1)
                        curr_fac = (-curr_fac.1, curr_fac.0)
                    }
                    Direction::Right => curr_fac = (curr_fac.1, -curr_fac.0),
                }
            }
        }
    }

    let facing_val = match curr_fac {
        (0, 1) => 0,  // right
        (0, -1) => 2, // left
        (1, 0) => 1,  // down
        (-1, 0) => 3, // up
        _ => unreachable!(),
    };

    // for (i, line) in all_lines.iter().enumerate() {
    //     for (j, char) in line.chars().enumerate() {
    //         if positions.contains(&(i, j)) {
    //             match facings[positions.iter().position(|&x| x == (i, j)).unwrap()] {
    //                 (0, 1) => print!(">"), // right
    //                 (0, -1) => print!("<"), // left
    //                 (1, 0) => print!("v"), // down
    //                 (-1, 0) => print!("^"), // up
    //                 _ => unreachable!()
    //             }
    //         } else {
    //             print!("{}", char);
    //         }
    //     }
    //     print!("\n");
    // }
    println!("{} {} {}", curr_pos.0 + 1, curr_pos.1 + 1, facing_val);
    println!(
        "{}",
        1000 * (curr_pos.0 + 1) + 4 * (curr_pos.1 + 1 + map[curr_pos.0].left_idx) + facing_val
    );
}

fn take_step(
    map: &Vec<MapRow>,
    curr_pos: (usize, usize),
    curr_fac: (isize, isize),
) -> (usize, usize) {
    let curr_row_width = map[curr_pos.0].tiles.len();
    let mut new_pos = curr_pos;
    if curr_fac.1 == -1 {
        // move left
        if curr_pos.1 == 0 {
            new_pos.1 = curr_row_width - 1;
        } else {
            new_pos.1 -= 1;
        }
    } else if curr_fac.1 == 1 {
        // move right
        if curr_pos.1 == curr_row_width - 1 {
            new_pos.1 = 0;
        } else {
            new_pos.1 += 1;
        }
    } else if curr_fac.0 == -1 {
        // move up
        let curr_row = &map[curr_pos.0];
        let mut above_row_offset = -1;
        loop {
            // find next row 'above' us
            let above_row_idx =
                (curr_pos.0 as isize + above_row_offset).rem_euclid(map.len() as isize) as usize;
            let above_row = &map[above_row_idx];
            // check if there is a tile in the 'above row'
            let abs_curr_col = curr_pos.1 + curr_row.left_idx;
            if (above_row.left_idx..above_row.right_idx).contains(&abs_curr_col) {
                new_pos = (above_row_idx, abs_curr_col - above_row.left_idx);
                break;
            }
            above_row_offset -= 1;
        }
    } else if curr_fac.0 == 1 {
        // move down
        let curr_row = &map[curr_pos.0];
        let mut below_row_offset = 1;
        loop {
            // find next row 'below' us
            let below_row_idx =
                (curr_pos.0 as isize + below_row_offset).rem_euclid(map.len() as isize) as usize;
            let below_row = &map[below_row_idx];
            // check if there is a tile in the 'below row'
            let abs_curr_col = curr_pos.1 + curr_row.left_idx;
            if (below_row.left_idx..below_row.right_idx).contains(&abs_curr_col) {
                new_pos = (below_row_idx, abs_curr_col - below_row.left_idx);
                break;
            }
            below_row_offset += 1;
        }
    }

    // check if new pos is blocked
    return if map[new_pos.0].tiles[new_pos.1] == 1 {
        // blocked
        curr_pos
    } else {
        // move
        new_pos
    };
}
