use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Tile {
    accessible: Vec<(usize, usize)>,
}

fn main() {
    let charmap = HashMap::from([
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
        ('k', 10),
        ('l', 11),
        ('m', 12),
        ('n', 13),
        ('o', 14),
        ('p', 15),
        ('q', 16),
        ('r', 17),
        ('s', 18),
        ('t', 19),
        ('u', 20),
        ('v', 21),
        ('w', 22),
        ('x', 23),
        ('y', 24),
        ('z', 25),
        ('E', 25),
        ('S', 0),
    ]);

    let mut map_raw: Vec<Vec<usize>> = vec![];
    let mut map: Vec<Vec<Tile>> = vec![];
    // Init input reader
    let file = File::open("day12_1.txt").unwrap();
    let filebuf = io::BufReader::new(file);
    let lineiter = filebuf.lines();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    // Read input into raw map
    for (i, line) in lineiter.enumerate() {
        let line = line.unwrap();
        let mut row = vec![];
        for (j, entry) in line.chars().enumerate() {
            if entry == 'S' {
                start = (i, j);
            } else if entry == 'E' {
                end = (i, j);
            }

            row.push(charmap[&entry]);
        }
        map_raw.push(row);
    }

    for (i, row) in map_raw.iter().enumerate() {
        let mut tile_row: Vec<Tile> = vec![];
        for (j, entry) in row.iter().enumerate() {
            let mut tile = Tile { accessible: vec![] };
            // check left and right
            if !(j == 0) {
                // check we are not on boundary
                let diff = map_raw[i][j - 1] as i32 - *entry as i32;
                if diff <= 1 {
                    tile.accessible.push((i, j - 1));
                }
            }
            if !(j == row.len() - 1) {
                let diff = map_raw[i][j + 1] as i32 - *entry as i32;
                if diff <= 1 {
                    tile.accessible.push((i, j + 1));
                }
            }
            // check up and down
            if !(i == 0) {
                let diff = map_raw[i - 1][j] as i32 - *entry as i32;
                if diff <= 1 {
                    tile.accessible.push((i - 1, j));
                }
            }
            if !(i == map_raw.len() - 1) {
                let diff = map_raw[i + 1][j] as i32 - *entry as i32;
                if diff <= 1 {
                    tile.accessible.push((i + 1, j));
                }
            }

            tile_row.push(tile);
        }
        map.push(tile_row);
    }

    let (visited, previous) = bfs(&map, start, end).unwrap();
    // Find path from end back to start
    let steps = get_pathlen(start, &visited, &previous);

    println!("Found exit in {} steps", steps - 1);

    // Part 2 --- this can almost certainly be made more efficient but I tried my best at it for
    // a while and stopped making progress. Doing a proper job would require a more serious overhaul
    // of how I handle my data structures, so I'm happy with this for now.
    let mut fewest_steps: usize = 100000;
    let mut fewest_coord: (usize, usize) = (0, 0);
    let mut already_tested: Vec<(usize, usize)> = vec![];
    for (i, row) in map.iter().enumerate() {
        'outer: for (j, entry) in row.iter().enumerate() {
            if map_raw[i][j] == charmap[&'a'] {
                if already_tested.contains(&(i, j)) {
                    // we already tested this square on another path, skip!
                    continue;
                };

                let mut new_route = false;
                for accessible in map[i][j].accessible.iter() {
                    // can I move somewhere that's not already been tested?
                    if !already_tested.contains(&accessible) {
                        new_route = true;
                    }
                }
                if !new_route {
                    continue;
                }

                let (visited, previous) = match bfs(&map, (i, j), end) {
                    Some(val) => val,
                    None => continue,
                };

                // Calculate route
                let route = get_route((i, j), &visited, &previous);
                // Find closest `a` to end on this route
                let mut furthest_a = route.len() - 1;
                for pos in route.iter() {
                    if map_raw[pos.0][pos.1] == charmap[&'a'] {
                        break;
                    }
                    furthest_a -= 1;
                }
                println!("Furthest a is {} steps from start", furthest_a);
                // Shortest path from any square on this route
                let steps = route.len() - furthest_a - 1;
                if steps < fewest_steps {
                    fewest_steps = steps;
                    fewest_coord = visited[furthest_a];
                }
                // Ignore all the steps we just visited in future checks
                already_tested.extend(route);
            }
        }
    }
    println!("{}, {:?}", fewest_steps, fewest_coord);
}

/// Given start and end coordinates, a vector of nodes visited, and a vector of the nodes the
/// visited nodes were visited from, find the length of the path from the final node visited to
/// the `start` node.
fn get_pathlen(
    start: (usize, usize),
    visited: &Vec<(usize, usize)>,
    previous: &Vec<usize>,
) -> usize {
    let route = get_route(start, visited, previous);
    route.len()
}

fn get_route(
    start: (usize, usize),
    visited: &Vec<(usize, usize)>,
    previous: &Vec<usize>,
) -> Vec<(usize, usize)> {
    let mut route: Vec<(usize, usize)> = vec![];
    let mut curr_node = (100000, 100000);
    let mut n = visited.len() - 1;

    let mut output: Vec<Vec<char>> = vec![vec!['.'; 180]; 50];

    while curr_node != start {
        curr_node = visited[n];
        route.push(curr_node);
        output[curr_node.0][curr_node.1] = '#';
        n = previous[n];
    }

    output[start.0][start.1] = 'S';

    for line in output.iter() {
        println!("{}", line.iter().collect::<String>());
    }

    route
}

/// Given `map`, find the shortest path from `start` to `end`, returning a vector of the positions
/// visited during the search, the position each of those positions was moved to from.
///
/// This would instantly be substantially faster if I used a meta index for the nodes of row*col
/// so that the calls to .contains() would be way, way faster. I could even keep the lists sorted
/// and use bisection. This was fast enough to solve the problem, though.
fn bfs(
    map: &Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(Vec<(usize, usize)>, Vec<usize>)> {
    // BFS traversal
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![]);
    queue.push_back(start);
    let mut visited: Vec<(usize, usize)> = vec![];
    let mut previous: Vec<usize> = vec![0];
    let mut n: usize = 0; // index for retrieving previous[end]
    loop {
        let curr_node = match queue.pop_front() {
            Some(node) => node,
            None => {
                // println!("Ran out of nodes");
                return None;
            }
        };

        if visited.contains(&curr_node) {
            previous.remove(n);
            continue;
        }
        visited.push((curr_node.0, curr_node.1));

        if curr_node == end {
            println!("Found exit at {:?}", end);
            break;
        }

        for accessible in map[curr_node.0][curr_node.1].accessible.iter() {
            if visited.contains(accessible) {
                continue;
            } else {
                queue.push_back(*accessible);
                previous.push(n);
            }
        }
        n += 1;
    }
    Some((visited, previous))
}
