use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_n(elf: &(isize, isize), elves: &HashSet<(isize, isize)>) -> bool {
    return !(elves.contains(&(elf.0 - 1, elf.1))
        || elves.contains(&(elf.0 - 1, elf.1 - 1))
        || elves.contains(&(elf.0 - 1, elf.1 + 1)));
}

fn check_s(elf: &(isize, isize), elves: &HashSet<(isize, isize)>) -> bool {
    return !(elves.contains(&(elf.0 + 1, elf.1))
        || elves.contains(&(elf.0 + 1, elf.1 - 1))
        || elves.contains(&(elf.0 + 1, elf.1 + 1)));
}

fn check_w(elf: &(isize, isize), elves: &HashSet<(isize, isize)>) -> bool {
    return !(elves.contains(&(elf.0, elf.1 - 1))
        || elves.contains(&(elf.0 - 1, elf.1 - 1))
        || elves.contains(&(elf.0 + 1, elf.1 - 1)));
}

fn check_e(elf: &(isize, isize), elves: &HashSet<(isize, isize)>) -> bool {
    return !(elves.contains(&(elf.0, elf.1 + 1))
        || elves.contains(&(elf.0 - 1, elf.1 + 1))
        || elves.contains(&(elf.0 + 1, elf.1 + 1)));
}

fn check_all(elf: &(isize, isize), elves: &HashSet<(isize, isize)>) -> bool {
    check_s(elf, elves) && check_n(elf, elves) && check_e(elf, elves) && check_w(elf, elves)
}

fn print_elves(
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize,
    elves: &Vec<(isize, isize)>,
) {
    for i in min_row..max_row + 1 {
        for j in min_col..max_col + 1 {
            if elves.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("-----------");
}

fn main() {
    // Init input reader
    let file = File::open(r"./src/data/day23_1.txt").unwrap();
    let filebuf = BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();
    //
    let mut elves: HashSet<(isize, isize)> = HashSet::new();
    for (i, line) in all_lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                elves.insert((i as isize, j as isize));
            }
        }
    }

    let checks: Vec<fn(&(isize, isize), &HashSet<(isize, isize)>) -> bool> =
        vec![check_n, check_s, check_w, check_e];
    let dirs: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    // iterate
    // part 1
    // for i in 0..10 {
    // part 2
    let mut i = 0;
    loop {
        let mut new_elves: HashSet<(isize, isize)> = HashSet::new();
        'outer: for elf in elves.iter() {
            if check_all(elf, &elves) {
                new_elves.insert(*elf);
                continue;
            } else {
                for j in 0..4 {
                    if (checks[(i + j) % 4])(&elf, &elves) {
                        let new_elf = (elf.0 + dirs[(i + j) % 4].0, elf.1 + dirs[(i + j) % 4].1);
                        if !new_elves.insert(new_elf) {
                            // insert was rejected, reset other elf and this elf
                            new_elves.insert(*elf);
                            new_elves.remove(&new_elf);
                            let old_elf = (
                                new_elf.0 + dirs[(i + j) % 4].0,
                                new_elf.1 + dirs[(i + j) % 4].1,
                            );
                            new_elves.insert(old_elf);
                        };
                        continue 'outer;
                    }
                }
                new_elves.insert(*elf);
            }
        }

        if new_elves == elves {
            println!("No elves moved on round {}", i + 1);
            break;
        }
        elves = new_elves;
        i += 1;
    }

    // sort by row
    let mut sorted_elves = elves.into_iter().collect::<Vec<(isize, isize)>>();
    sorted_elves.sort_by(|a, b| a.0.cmp(&b.0));
    let min_row = sorted_elves[0].0;
    let max_row = sorted_elves.last().unwrap().0;
    let width = max_row - min_row;
    sorted_elves.sort_by(|a, b| a.1.cmp(&b.1));
    let min_col = sorted_elves[0].1;
    let max_col = sorted_elves.last().unwrap().1;
    let height = max_col - min_col;
    print_elves(min_row, max_row, min_col, max_col, &sorted_elves);

    println!(
        "{}",
        (height + 1) * (width + 1) - sorted_elves.len() as isize
    );
}
