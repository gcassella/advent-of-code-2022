use std::{
    fs::File,
    io::{self, BufRead},
};

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

/// Iterate forwards and backwards through each row of the grid, storing the
/// indices of trees that are visible from either end
fn count_talltrees_leftright(
    grid: &Vec<Vec<usize>>,
    mut preseen: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let rowlen = grid[0].len();
    let mut seen: Vec<(usize, usize)> = vec![];
    seen.append(&mut preseen);
    for (i, row) in grid.iter().enumerate() {
        let mut largest: usize = 0;
        for (j, val) in row.iter().enumerate() {
            if val > &largest {
                largest = *val;
                if !seen.contains(&(i, j)) {
                    seen.push((i, j));
                }
            }
        }

        largest = 0;
        for (j, val) in row.iter().rev().enumerate() {
            let k = rowlen - j - 1;
            if val > &largest {
                largest = *val;
                if !seen.contains(&(i, k)) {
                    seen.push((i, k));
                }
            }
        }
    }
    seen
}

/// Returns the viewing distance of the tree at site (i, j). I simply
/// brute force this by walking through the array in each direction and adding
/// up the number of seen trees.
fn get_viewing_distance(grid: &Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let my_size = grid[i][j];
    let mut total = 1;
    if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[0].len() - 1 {
        return 0;
    }
    // Walk up
    let mut curr_i = i - 1;
    let mut other_size = 0;
    let mut accumulator = 0;
    while other_size < my_size {
        accumulator += 1;
        other_size = grid[curr_i][j];
        if curr_i == 0 {
            break;
        } else {
            curr_i -= 1;
        }
    }
    total *= accumulator;
    accumulator = 0;
    // Walk down
    let mut curr_i = i + 1;
    let mut other_size = 0;
    while other_size < my_size {
        accumulator += 1;
        other_size = grid[curr_i][j];
        if curr_i == grid.len() - 1 {
            break;
        } else {
            curr_i += 1;
        }
    }
    total *= accumulator;
    accumulator = 0;
    // Walk left
    let mut curr_j = j - 1;
    let mut other_size = 0;
    while other_size < my_size {
        accumulator += 1;
        other_size = grid[i][curr_j];
        if curr_j == 0 {
            break;
        } else {
            curr_j -= 1;
        }
    }
    total *= accumulator;
    accumulator = 0;
    // Walk down
    let mut curr_j = j + 1;
    let mut other_size = 0;
    while other_size < my_size {
        accumulator += 1;
        other_size = grid[i][curr_j];
        if curr_j == grid[0].len() - 1 {
            break;
        } else {
            curr_j += 1;
        }
    }
    total *= accumulator;

    total
}

/// I spent a while thinking hard about clever ways to do this. It was a waste
/// of time. The stupid way will work.
fn main() {
    // Init input reader
    let file = File::open("day8_1.txt").unwrap();
    let filebuf = io::BufReader::new(file);
    let lineiter = filebuf.lines();

    let grid = lineiter
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize + 1) // Adding 1 to everything so I can use 0 as my default
                .collect::<Vec<usize>>() // for the 'largest' comparator later.
        })
        .collect::<Vec<Vec<usize>>>();

    // Accumulate a list of co-ordinates of visible trees seen by looking
    // along rows of the grid
    let seen_trees = count_talltrees_leftright(&grid, vec![]);
    println!("{}", seen_trees.len());
    // Transpose the grid and do the same, doing some bookkeeping to avoid
    // recounting trees we already saw.
    let grid_t = transpose(&grid);
    let seen_trees_t: Vec<(usize, usize)> = seen_trees.iter().map(|(i, j)| (*j, *i)).collect();
    let final_trees = count_talltrees_leftright(&grid_t, seen_trees_t);
    // Print number of seen trees
    println!("{}", final_trees.len());
    // This is brute force, but it works.
    let mut highest_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let dist = get_viewing_distance(&grid, i, j);
            if dist > highest_score {
                highest_score = dist;
            }
        }
    }
    println!("{}", highest_score);
}
