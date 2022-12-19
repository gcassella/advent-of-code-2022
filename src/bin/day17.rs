use std::fs;

enum Move {
    Left,
    Right,
}

struct Piece {
    rocks: Vec<(i32, i32)>,
    height: usize,
}

static METASTATE_DEPTH: usize = 1000;

/// Utility for transposing vectors of vectors in[i][j] -> out[j][i]
fn transpose(mat: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut out = vec![vec![0; mat.len()]; mat[0].len()];

    for (i, row) in mat.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            out[j][i] = entry.clone();
        }
    }

    out
}

fn print_state(state: &Vec<Vec<usize>>) {
    let trans_state = transpose(&state);
    for row in trans_state {
        println!(
            "{}",
            row.iter()
                .map(|x| match x {
                    0 => '.',
                    1 => '#',
                    _ => '#',
                })
                .collect::<String>()
        )
    }
}

/// Add/remove empty rows to/from state to ensure that piece is inserted at the correct position
/// (bottom left corner of piece bounding box three columns from left edge, three rows from
/// uppermost rock in state).
fn resize_for_piece(
    state: &Vec<Vec<usize>>,
    floor_level: &Vec<usize>,
    piece: &Piece,
) -> Vec<Vec<usize>> {
    let mut new_state = state.clone();

    let mut max_diff: i32 = -1000;
    for level in floor_level.iter() {
        let diff: i32 = piece.height as i32 + 3 - *level as i32;
        if diff > max_diff {
            max_diff = diff;
        }
    }

    for col in new_state.iter_mut() {
        if max_diff > 0 {
            // need to add head room
            for _ in 0..max_diff {
                col.insert(0, 0);
            }
        } else if max_diff < 0 {
            // need to remove head room
            *col = col
                .split_at((max_diff * -1) as usize)
                .1
                .iter()
                .map(|x| *x)
                .collect();
        }
    }

    new_state
}

/// Given a state, a piece, and a position, check if any of the rocks in the piece intersect
/// with any of the rocks in the state (and return true), else false.
fn check_for_collision(state: &Vec<Vec<usize>>, piece: &Piece, piece_pos: (usize, usize)) -> bool {
    for rock in piece.rocks.iter() {
        let rock_pos = (piece_pos.0 as i32 + rock.0, piece_pos.1 as i32 + rock.1);
        // check move puts us within bounds
        if rock_pos.0 < 0 || rock_pos.0 > 6 {
            return true; // collided with boundary
        }
        if rock_pos.1 < 0 {
            // should never have been able to move upwards???
            unreachable!()
        }

        if state[rock_pos.0 as usize][rock_pos.1 as usize] == 1 {
            return true;
        }
    }

    false
}

/// We will assume piece_pos is given such that casting rock_pos to usize and indexing state will
/// not cause an indexing error. This is a possible source of runtime panics.
fn add_piece_to_state(state: &mut Vec<Vec<usize>>, piece: &Piece, piece_pos: (usize, usize)) {
    for rock in piece.rocks.iter() {
        let rock_pos = (piece_pos.0 as i32 + rock.0, piece_pos.1 as i32 + rock.1);
        state[rock_pos.0 as usize][rock_pos.1 as usize] = 1;
    }
}

/// Given a state, return a vector with an entry for each column of the state with the y-index of
/// the first rock encountered.
fn get_floor_level(state: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut floor_level: Vec<usize> = vec![];
    for col in state.iter() {
        let mut col_level = 0;
        for entry in col {
            if *entry == 0 {
                col_level += 1;
                continue;
            } else {
                break;
            }
        }
        floor_level.push(col_level);
    }
    floor_level
}

fn drop_piece(
    moves: &Vec<Move>,
    state: &mut Vec<Vec<usize>>,
    curr_piece: &Piece,
    init_counter: usize,
) -> usize {
    let mut counter = init_counter;
    let mut piece_pos = (2, curr_piece.height - 1);
    loop {
        if counter % 2 == 0 {
            // blow
            let curr_move = &moves[(counter / 2) % moves.len()];
            match curr_move {
                Move::Left => {
                    // print!("Blowing piece left");
                    if piece_pos.0 == 0 {
                        // print!(", but nothing happened\n")
                    } else if !check_for_collision(
                        &state,
                        curr_piece,
                        (piece_pos.0 - 1, piece_pos.1),
                    ) {
                        piece_pos.0 = piece_pos.0 - 1;
                        // print!("\n");
                    } else {
                        // print!(", but nothing happened\n")
                    }
                }
                Move::Right => {
                    // print!("Blowing piece right");
                    if !check_for_collision(&state, curr_piece, (piece_pos.0 + 1, piece_pos.1)) {
                        piece_pos.0 = piece_pos.0 + 1;
                        // print!("\n");
                    } else {
                        // print!(", but nothing happened\n")
                    }
                }
            };
        } else {
            // drop
            if !check_for_collision(&state, curr_piece, (piece_pos.0, piece_pos.1 + 1)) {
                // print!("Piece falls\n");
                piece_pos.1 = piece_pos.1 + 1;
            } else {
                add_piece_to_state(state, curr_piece, piece_pos);
                break;
            }
        }
        counter += 1;
    }
    counter
}

/// Convert state to more compact representation. Each row in the state is a vector of seven bits
/// which are either 0 or 1, we can pack these naturally into a u8 and represent the state as a
/// Vec<u8>
fn state_to_bits(state: &Vec<Vec<usize>>) -> Vec<u8> {
    let state_trans = transpose(&state);
    let mut out: Vec<u8> = vec![];
    for row in state_trans.iter() {
        let val = row.iter().fold(0, |val, &entry| (val << 1) + entry as u8);
        out.push(val);
    }
    out
}

fn main() {
    // Init input reader
    let input = fs::read_to_string(r".\src\data\day17_1.txt").expect("Error opening file");
    let moves: Vec<Move> = input
        .chars()
        .map(|x| match x {
            '>' => Move::Right,
            '<' => Move::Left,
            _ => unreachable!(),
        })
        .collect();
    // define pieces
    let pieces = vec![
        Piece {
            rocks: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            height: 1,
        },
        Piece {
            rocks: vec![(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)],
            height: 3,
        },
        Piece {
            rocks: vec![(0, 0), (1, 0), (2, 0), (2, -1), (2, -2)],
            height: 3,
        },
        Piece {
            rocks: vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            height: 4,
        },
        Piece {
            rocks: vec![(0, 0), (1, 0), (0, -1), (1, -1)],
            height: 2,
        },
    ];

    // our y coordinate system will be increasing downwards, with the zero level at the current
    // rock insertion point
    let mut floor_level: Vec<usize> = vec![3; 7];
    // state[x][y] = 1 if there is a rock, 0 otherwise
    let mut state: Vec<Vec<usize>> = vec![];
    for level in floor_level.iter() {
        let mut col = vec![0; *level as usize];
        col.push(1); // add the floor
        state.push(col);
    }

    let mut counter = 0;
    let mut curr_piece = &pieces[0];

    // store metastate as top 100 rows, piece dropping, and jet blowing
    let mut prev_metastates: Vec<(Vec<u8>, usize, usize)> = vec![];
    // store height of the tower at each iteration to do the cycle-height arithmetic later
    let mut heights: Vec<usize> = vec![];

    for i in 0..1000000000000 {
        curr_piece = &pieces[i % pieces.len()];
        // add or remove headroom (alternatively viewed, set y=0 to the right place) for size
        // of piece
        state = resize_for_piece(&state, &floor_level, curr_piece);
        // tetris the piece according to the movelist
        counter = drop_piece(&moves, &mut state, curr_piece, counter);
        // if the clock is on a 'drop' phase, go to a 'blow' phase, as all pieces are blown to
        // begin with
        if counter % 2 == 1 {
            counter += 1;
        }
        // update list of highest rock in each column, used for setting the y=0 level appropriately
        floor_level = get_floor_level(&state);

        // Define our metastate by the state of the top METASTATE_DEPTH rows, where we are in the
        // piece cycle, and where we are in the jet cycle. METASTATE_DEPTH is heuristic, and
        // this method is vulnerable to adversarial inputs, but it works for the (thankfully tame)
        // input given by advent of code
        let metastate = (
            state_to_bits(&state)[..i.min(METASTATE_DEPTH)].to_vec(),
            i % pieces.len(),
            (counter / 2) % moves.len(),
        );
        // When the metastate repeats,
        if prev_metastates.contains(&metastate) {
            let idx = prev_metastates
                .iter()
                .position(|x| *x == metastate)
                .unwrap();
            let cycle_len = i - idx;
            let num_cycles = (1000000000000 - idx) / cycle_len;
            let rem_cycles = (1000000000000 - idx) % cycle_len;

            let cycle_height = state[0].len() - 1 - 3 - curr_piece.height - heights[idx];

            println!(
                "Found cycle after {} iterations, beginning at {}, with length {} and height {}",
                i,
                idx,
                i - idx,
                cycle_height
            );
            println!(
                "The additional height from the cycles out to the end is {}",
                num_cycles * cycle_height
            );
            println!(
                "The remainder is the height gained rem_cycles into a cycle which is {}",
                heights[idx + rem_cycles]
            );
            println!(
                "Yielding a final height of {}",
                num_cycles * cycle_height + heights[idx + rem_cycles]
            );
            break;
        }
        prev_metastates.push(metastate);

        let height = state[0].len() - 1 - 3 - curr_piece.height;
        heights.push(height);
    }

    // previously required for part 1
    // println!("Height after 2022 rocks is {}", heights[2021]);
}
