use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Wall {
    Horizontal(
        (usize, usize),
        (usize, usize)
    ),
    Vertical(
        (usize, usize),
        (usize, usize)
    )
}


/// a - b
fn subtract_coord(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0 - b.0, a.1 - b.1)
}


/// Given a string "\d,\d", return (usize, usize)
fn parse_string_to_coord(input: &String) -> (usize, usize) {
    let vals: Vec<usize> = input.split(",").map(|x| x.parse().unwrap()).collect();
    (vals[0], vals[1])
}


fn parse_string_to_walls(input: &String, xoffset: usize, yoffset: usize) -> Vec<Wall> {
    let origin = (xoffset, yoffset);
    lazy_static! {
        static ref COORD_RE: Regex = Regex::new(r"(\d+,\d+)").unwrap();
    }

    let coords: Vec<(usize, usize)> = COORD_RE
        .captures_iter(&input[..])
        .map(|x| {
            let val = x.get(1).unwrap().as_str();
            parse_string_to_coord(&String::from(val))
        })
        .collect();

    let mut walls: Vec<Wall> = vec![];
    let mut coord_it = coords.iter().peekable();
    while let Some(coord) = coord_it.next() {
        let peek_next_coord = coord_it.peek();
        let next_coord = match peek_next_coord.as_deref() {
            Some(&val) => val,
            None => break
        };
        if next_coord.0 == coord.0 {
            // Wall is vertical
            walls.push(Wall::Vertical(
                subtract_coord(*coord, origin),
                subtract_coord(*next_coord, origin)
            )
            );
        } else {
            // Wall is horizontal
            walls.push(Wall::Horizontal(
                subtract_coord(*coord, origin),
                subtract_coord(*next_coord, origin)
            )
            );
        }
    }

    walls
}


fn add_wall_to_map(map: &mut Vec<Vec<bool>>, wall: Wall) {
    match wall {
        Wall::Horizontal(start, end) => {
            let y = start.1;
            for i in if start.0 < end.0 { start.0..end.0+1 } else { end.0..start.0+1 } {
                map[y][i] = true;
            }
        },
        Wall::Vertical(start, end) => {
            let x = start.0;
            for i in if start.1 < end.1 { start.1..end.1+1 } else { end.1..start.1+1 } {
                map[i][x] = true;
            }
        }
    }
}


fn print_map(map: &Vec<Vec<bool>>) {
    for row in map {
        for entry in row {
            if *entry {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}


/// Returns true if sand is added to the map succesfully, false otherwise (sand falls off).
/// Also return false if sand gets stuck at the entry point
fn drop_sand(map: &mut Vec<Vec<bool>>, entry_point: usize) -> bool {
    let mut curr_pos = (entry_point, 0);
    if map[curr_pos.1][curr_pos.0] { // entry point blocked
        return false
    }
    loop {
        if map[curr_pos.1+1][curr_pos.0] { // cell below blocked
            if curr_pos.0 == 0 { // handle left boundary, fall off left
                return false
            }
            if map[curr_pos.1+1][curr_pos.0-1] { // cell left blocked
                if curr_pos.0 == map[0].len() - 1 { // handle right boundary, fall off right

                }
                if map[curr_pos.1+1][curr_pos.0+1] { // cell right blocked, stick
                    map[curr_pos.1][curr_pos.0] = true;
                    println!("Added sand at {} {}", curr_pos.0, curr_pos.1);
                    return true
                } else { // cell right unblocked, fall
                    curr_pos = (curr_pos.0+1, curr_pos.1+1)
                }
            } else { // cell left unblocked, fall
                curr_pos = (curr_pos.0-1, curr_pos.1+1)
            }
        } else { // cell below unblocked, fall
            curr_pos = (curr_pos.0, curr_pos.1+1)
        }
    }
    true
}


fn main() {
    lazy_static! {
        static ref H_RE: Regex = Regex::new(r"(\d+),\d+").unwrap();
        static ref V_RE: Regex = Regex::new(r"\d+,(\d+)").unwrap();
    }
    // Init input reader
    let file = File::open(r".\src\data\day14_1.txt").unwrap();
    let filebuf = BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();
    // Loop through lines and populate array of walls
    // we also want to find the max/min extent of the map in each direction while we're at it
    let mut walls: Vec<Wall> = vec![];
    let mut x_max = 0;
    let mut x_min = 10000;
    let mut y_max = 0;
    let mut y_min = 0;
    for line in all_lines.iter() {
        for hval in H_RE.captures_iter(&line[..]) {
            let val = hval.get(1).unwrap().as_str().parse().unwrap();
            if val > x_max {
                x_max = val;
            }
            if val < x_min {
                x_min = val;
            }
        }

        for vval in V_RE.captures_iter(&line[..]) {
            let val = vval.get(1).unwrap().as_str().parse().unwrap();
            if val > y_max {
                y_max = val;
            }
            if val < y_min {
                y_min = val;
            }
        }
    }
    println!("x in ({}, {}), y in ({}, {})", x_min, x_max, y_min, y_max);

    let height = 2 + y_max - y_min;
    let width = height * 2 + (x_max - x_min);

    for line in all_lines.iter() {
        walls.append(&mut parse_string_to_walls(&line, x_min - height, y_min));
    }

    // Loop over array of walls and construct map
    let mut map = vec![vec![false; width + 1]; height + 1];
    for wall in walls {
        println!("{:?}", wall);
        add_wall_to_map(&mut map, wall);
    }
    // add infinite wall
    add_wall_to_map(
        &mut map,
        Wall::Horizontal(
            (0, height),
            (width, height)
        )
    );

    //
    let mut units = 0;
    let entry_point = 500 - x_min + height;
    while drop_sand(&mut map, entry_point) {
        units += 1;
    }
    print_map(&map);
    println!("{}", units);
}