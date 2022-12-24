use ndarray::{s, Array3, ArrayBase, Ix3, OwnedRepr};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

// this must be greater than the largest t state we visit
// this can be made generic by precalculating LCM(rows, cols) steps
// and taking the remainder of t before indexing map, but i cba
static PRECALCULATED_STEPS: usize = 1500;
type Blizz = (usize, usize, usize);

fn draw_storm(blizzards: &Vec<Blizz>, bounds: (usize, usize, usize, usize)) {
    for i in bounds.0..bounds.2 + 1 {
        for j in bounds.3..bounds.1 + 1 {
            if blizzards.contains(&(i, j, 0)) {
                print!("^");
            } else if blizzards.contains(&(i, j, 1)) {
                print!(">");
            } else if blizzards.contains(&(i, j, 2)) {
                print!("v");
            } else if blizzards.contains(&(i, j, 3)) {
                print!("<");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("-----------------------------------");
}

fn main() {
    // Init input reader
    let file = File::open(r"./src/data/day24_1.txt").unwrap();
    let filebuf = BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();

    // row, col, direction, 0 = up, 1 = right, 2 = down, 3 = left
    let mut blizzards: Vec<Blizz> = vec![];
    let bounds = (0, all_lines[0].len() - 1, all_lines.len() - 1, 0);
    for (i, line) in all_lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '^' {
                blizzards.push((i, j, 0));
            } else if char == '>' {
                blizzards.push((i, j, 1));
            } else if char == 'v' {
                blizzards.push((i, j, 2));
            } else if char == '<' {
                blizzards.push((i, j, 3));
            }
        }
    }

    let mut map = Array3::<usize>::zeros([PRECALCULATED_STEPS, bounds.2 + 1, bounds.1 + 1]);
    for t in 0..PRECALCULATED_STEPS {
        // println!("Step {}", t);
        // draw_storm(&blizzards, bounds);
        for blizz in blizzards.iter_mut() {
            map[(t, blizz.0, blizz.1)] = 1;
            match blizz.2 {
                0 => {
                    let mut new_row = blizz.0 - 1;
                    if new_row == bounds.0 {
                        new_row = bounds.2 - 1;
                    }
                    blizz.0 = new_row;
                } // move up
                1 => {
                    let mut new_col = blizz.1 + 1;
                    if new_col == bounds.1 {
                        new_col = bounds.3 + 1;
                    }
                    blizz.1 = new_col;
                } // move right
                2 => {
                    let mut new_row = blizz.0 + 1;
                    if new_row == bounds.2 {
                        new_row = bounds.0 + 1;
                    }
                    blizz.0 = new_row;
                } // move down
                3 => {
                    let mut new_col = blizz.1 - 1;
                    if new_col == bounds.3 {
                        new_col = bounds.1 - 1;
                    }
                    blizz.1 = new_col;
                } // move left
                _ => unreachable!(),
            }
        }
    }

    // pathfind through map with bfs
    let init_pos = (0, 0, 1);
    let exit = (bounds.2, bounds.1 - 1);

    let exit_t = set_bfs(bounds, &map, init_pos, exit);

    // pathfind back through map with bfs
    let init_pos = (exit_t, exit.0, exit.1);
    let exit = (0, 1);

    let entrance_t = set_bfs(bounds, &map, init_pos, exit);

    // pathfind AGAIN back through map with bfs
    let init_pos = (entrance_t, 0, 1);
    let exit = (bounds.2, bounds.1 - 1);

    let final_t = set_bfs(bounds, &map, init_pos, exit);

    println!(
        "Reached exit in {} steps, entrance in {} steps, and exit again in {} steps",
        exit_t,
        entrance_t - exit_t,
        final_t - entrance_t
    );
    println!("Total time taken {}", final_t);
}

fn set_bfs(
    bounds: (usize, usize, usize, usize),
    map: &ArrayBase<OwnedRepr<usize>, Ix3>,
    init_pos: (usize, usize, usize),
    exit: (usize, usize),
) -> usize {
    let mut queue: HashSet<(usize, usize, usize)> = HashSet::from([init_pos]);
    let mut exit_t = 0;
    'outer: loop {
        let curr_positions: Vec<(usize, usize, usize)> = queue.drain().collect();
        for curr_pos in curr_positions {
            if (curr_pos.1, curr_pos.2) == exit {
                println!("Exit found after {} steps", curr_pos.0);
                exit_t = curr_pos.0;
                break 'outer;
            };
            // wait where we are
            let next_pos = (curr_pos.0 + 1, curr_pos.1, curr_pos.2);
            if map[next_pos] == 0 {
                queue.insert(next_pos);
            }
            // move down
            let next_pos = (curr_pos.0 + 1, curr_pos.1 + 1, curr_pos.2);
            if next_pos.1 < bounds.2 && map[next_pos] == 0 {
                queue.insert(next_pos);
            }
            // special case for entering exit
            if (next_pos.1, next_pos.2) == exit && map[next_pos] == 0 {
                queue.insert(next_pos);
            }
            // move right
            let next_pos = (curr_pos.0 + 1, curr_pos.1, curr_pos.2 + 1);
            // move is valid
            // if we're not at the entrance, hitting the bound, or hitting a blizzard
            if next_pos.1 > 0
                && next_pos.1 < bounds.2
                && next_pos.2 < bounds.1
                && map[next_pos] == 0
            {
                queue.insert(next_pos);
            }
            // move left
            let next_pos = (curr_pos.0 + 1, curr_pos.1, curr_pos.2 - 1);
            // move is valid
            // if we're not at the entrance or exit row, hitting the bound, or hitting a blizzard
            if next_pos.1 > 0
                && next_pos.1 < bounds.2
                && next_pos.2 > bounds.3
                && map[next_pos] == 0
            {
                queue.insert(next_pos);
            }
            // move up
            if curr_pos.1 > 0 {
                // account for being stood at entrance
                let next_pos = (curr_pos.0 + 1, curr_pos.1 - 1, curr_pos.2);
                if next_pos.1 > bounds.0 && map[next_pos] == 0 {
                    queue.insert(next_pos);
                }
                // special case for entering entrance
                if (next_pos.1, next_pos.2) == (0, 1) && map[next_pos] == 0 {
                    queue.insert(next_pos);
                }
            }
        }
    }
    exit_t
}
