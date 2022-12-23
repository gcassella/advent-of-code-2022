use std::fs::File;
use std::io::{BufRead, BufReader};

static FACE_SIZE: usize = 50;

#[derive(Clone)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

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

/// (desitnation face, side)
#[derive(Clone)]
struct Face {
    id: usize,
    up: (usize, Side),
    down: (usize, Side),
    left: (usize, Side),
    right: (usize, Side),
}

#[derive(Clone)]
struct Position {
    face: Face,
    row: usize,
    col: usize,
}

fn main() {
    // Init input reader
    let file = File::open(r"./src/data/day22_1.txt").unwrap();
    let filebuf = BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();
    //
    let mut walls: Vec<(usize, usize)> = vec![];
    let mut lineiter = all_lines.iter();
    for (row, line) in lineiter.enumerate() {
        if line.is_empty() {
            break;
        }

        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                walls.push((row, col));
            }
        }
    }
    let mut instructions = all_lines.last().unwrap().clone();
    //
    let faces = vec![
        Face {
            id: 0,
            up: (5, Side::Bottom),
            left: (3, Side::Left),
            down: (2, Side::Top),
            right: (1, Side::Left),
        },
        Face {
            id: 1,
            up: (5, Side::Bottom),
            left: (0, Side::Right),
            down: (2, Side::Right),
            right: (4, Side::Right),
        },
        Face {
            id: 2,
            up: (0, Side::Bottom),
            down: (4, Side::Top),
            left: (3, Side::Top),
            right: (1, Side::Bottom),
        },
        Face {
            id: 3,
            up: (2, Side::Left),
            down: (5, Side::Top),
            right: (4, Side::Left),
            left: (0, Side::Left),
        },
        Face {
            id: 4,
            up: (2, Side::Bottom),
            down: (5, Side::Right),
            left: (3, Side::Right),
            right: (1, Side::Right),
        },
        Face {
            id: 5,
            up: (3, Side::Bottom),
            down: (1, Side::Top),
            right: (4, Side::Bottom),
            left: (0, Side::Top),
        },
    ];
    let mut curr_pos: Position = Position {
        face: faces[0].clone(),
        row: 0,
        col: FACE_SIZE,
    };
    let mut curr_fac: (isize, isize) = (0, 1);

    let mut positions: Vec<(usize, usize)> = vec![];
    let mut facings: Vec<(isize, isize)> = vec![];
    //

    while instructions.len() > 0 {
        let instruction = next_instruction(&mut instructions).unwrap();
        match instruction {
            // These conversions can underflow given a malformed input.
            Instruction::Move(steps) => {
                for _ in 0..steps {
                    (curr_pos, curr_fac) = take_step(&walls, &faces, curr_pos.clone(), curr_fac);
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

    let pos = face_pos_to_map_pos(&curr_pos);
    println!("{} {} {}", pos.0 + 1, pos.1 + 1, facing_val);
    println!("{}", 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + facing_val);
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

fn face_pos_to_map_pos(curr_pos: &Position) -> (usize, usize) {
    let origins = vec![
        (FACE_SIZE, 0),
        (2 * FACE_SIZE, 0),
        (FACE_SIZE, FACE_SIZE),
        (0, 2 * FACE_SIZE),
        (FACE_SIZE, 2 * FACE_SIZE),
        (0, 3 * FACE_SIZE),
    ];
    let origin = origins[curr_pos.face.id];
    (origin.0 + curr_pos.row, origin.1 + curr_pos.col)
}

fn take_step(
    walls: &Vec<(usize, usize)>,
    faces: &Vec<Face>,
    curr_pos: Position,
    curr_fac: (isize, isize),
) -> (Position, (isize, isize)) {
    let mut new_coord: (isize, isize) = (
        curr_pos.row as isize + curr_fac.0,
        curr_pos.col as isize + curr_fac.1,
    );
    let mut new_pos: Position = curr_pos.clone();
    let mut new_fac: (isize, isize) = (0, 0);
    if new_coord.0 < 0 {
        // move off left of face
        let (new_face_id, new_face_side) = &curr_pos.face.left;
        match new_face_side {
            Side::Left => {
                new_pos.col = 0;
                new_pos.row = FACE_SIZE - 1 - new_pos.row;
                new_fac.1 = 1;
            }
            Side::Right => {
                new_pos.col = FACE_SIZE - 1;
                new_fac.1 = -1;
            }
            Side::Top => {
                new_pos.col = new_pos.row;
                new_pos.row = 0;
                new_fac.0 = 1;
            }
            Side::Bottom => {
                new_pos.col = new_pos.row;
                new_pos.row = FACE_SIZE - 1;
                new_fac.1 = -1;
            }
        }
        new_pos.face = faces[*new_face_id].clone();
    } else if new_coord.0 >= FACE_SIZE as isize {
        // move off right of face
        let (new_face_id, new_face_side) = &curr_pos.face.right;
        match new_face_side {
            Side::Left => {
                new_pos.col = 0;
                new_fac.1 = 1;
            }
            Side::Right => {
                new_pos.col = FACE_SIZE - 1;
                new_pos.row = FACE_SIZE - 1 - new_pos.row;
                new_fac.1 = -1;
            }
            Side::Top => {
                new_pos.col = new_pos.row;
                new_pos.row = 0;
                new_fac.0 = 1;
            }
            Side::Bottom => {
                new_pos.col = new_pos.row;
                new_pos.row = FACE_SIZE - 1;
                new_fac.1 = -1;
            }
        }
        new_pos.face = faces[*new_face_id].clone();
    } else if new_coord.1 < 0 {
        // move off top of face
        let (new_face_id, new_face_side) = &curr_pos.face.up;
        match new_face_side {
            Side::Left => {
                new_pos.row = new_pos.col;
                new_pos.col = 0;
                new_fac.1 = 1;
            }
            Side::Right => {
                new_pos.row = new_pos.col;
                new_pos.col = FACE_SIZE - 1;
                new_fac.1 = -1;
            }
            Side::Top => {
                new_pos.row = 0;
                new_pos.col = FACE_SIZE - 1 - new_pos.col;
                new_fac.0 = 1;
            }
            Side::Bottom => {
                new_pos.row = FACE_SIZE - 1;
                new_fac.1 = -1;
            }
        }
        new_pos.face = faces[*new_face_id].clone();
    } else if new_coord.1 >= FACE_SIZE as isize {
        // move off bottom of face
        let (new_face_id, new_face_side) = &curr_pos.face.down;
        match new_face_side {
            Side::Left => {
                new_pos.row = new_pos.col;
                new_pos.col = 0;
                new_fac.1 = 1;
            }
            Side::Right => {
                new_pos.row = new_pos.col;
                new_pos.col = FACE_SIZE - 1;
                new_fac.1 = -1;
            }
            Side::Top => {
                new_pos.row = 0;
                new_fac.0 = 1;
            }
            Side::Bottom => {
                new_pos.row = FACE_SIZE - 1;
                new_pos.col = FACE_SIZE - 1 - new_pos.col;
                new_fac.1 = -1;
            }
        }
        new_pos.face = faces[*new_face_id].clone();
    } else {
        new_pos.row = (new_pos.row as isize + curr_fac.0) as usize;
        new_pos.col = (new_pos.col as isize + curr_fac.1) as usize;
        new_fac = curr_fac;
    }

    let map_pos = face_pos_to_map_pos(&new_pos);

    // check if new pos is blocked
    return if walls.contains(&map_pos) {
        // blocked
        (curr_pos, curr_fac)
    } else {
        // move
        (new_pos, new_fac)
    };
}
