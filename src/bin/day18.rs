use std::collections::VecDeque;
use std::fs;

/// upper bound on the extent of the droplet, found by eye but easily automated
static EXTENT: usize = 32;

fn coord_to_idx(x: usize, y: usize, z: usize) -> usize {
    z * EXTENT * EXTENT + y * EXTENT + x
}

/// Given the coordinate of a cube, check if occupied contains a cube at each neighbouring coord
/// and return the number of open faces
fn open_faces(occupied: &Vec<usize>, idx: usize) -> usize {
    let mut open: usize = 6;

    if occupied.contains(&(idx + 1)) {
        open -= 1;
    }
    if idx > 0 && occupied.contains(&(idx - 1)) {
        open -= 1;
    }
    if occupied.contains(&(idx + EXTENT)) {
        open -= 1;
    }
    if idx >= EXTENT && occupied.contains(&(idx - EXTENT)) {
        open -= 1;
    }
    if occupied.contains(&(idx + EXTENT * EXTENT)) {
        open -= 1;
    }
    if idx >= EXTENT * EXTENT && occupied.contains(&(idx - EXTENT * EXTENT)) {
        open -= 1;
    }
    open
}

fn main() {
    // Init input reader
    let input = fs::read_to_string(r".\src\data\day18_1.txt").expect("Error opening file");

    let occupied: Vec<usize> = input
        .split("\r\n") // sorry, unix users
        .map(|x| {
            let coord: Vec<usize> = x.split(",").map(|y| y.parse().unwrap()).collect();
            coord_to_idx(coord[0], coord[1], coord[2])
        })
        .collect();
    let mut total_open_faces = 0;
    for cube in occupied.iter() {
        total_open_faces += open_faces(&occupied, *cube);
    }

    println!("{}", total_open_faces);

    // find every point accessible from (0, 0, 0) using bfs
    // if this point neighbours a cube (open faces of this point < 6), tally up its CLOSED faces
    let starting_pos: usize = 0;
    let mut visited: Vec<usize> = vec![];
    let mut queue: VecDeque<usize> = VecDeque::from(vec![0]);

    let mut total_surface_area = 0;
    let mut total_visited = 0;
    while queue.len() > 0 {
        let curr_idx = queue.pop_front().unwrap();
        total_visited += 1;
        visited.push(curr_idx);
        let closed_faces = 6 - open_faces(&occupied, curr_idx);
        total_surface_area += closed_faces;

        // push all neighbouring points onto queue, neighbours only in orthogonal directions
        if curr_idx < EXTENT * EXTENT * EXTENT - 1 {
            let new_idx = curr_idx + 1;
            if !visited.contains(&new_idx)
                && !occupied.contains(&new_idx)
                && !queue.contains(&new_idx)
            {
                queue.push_back(new_idx);
            }
        }
        if curr_idx < EXTENT * EXTENT * EXTENT - EXTENT {
            let new_idx = curr_idx + EXTENT;
            if !visited.contains(&new_idx)
                && !occupied.contains(&new_idx)
                && !queue.contains(&new_idx)
            {
                queue.push_back(new_idx);
            }
        }
        if curr_idx < EXTENT * EXTENT * EXTENT - EXTENT * EXTENT {
            let new_idx = curr_idx + EXTENT * EXTENT;
            if !visited.contains(&new_idx)
                && !occupied.contains(&new_idx)
                && !queue.contains(&new_idx)
            {
                queue.push_back(new_idx);
            }
        }

        if curr_idx > 0 {
            let new_idx = curr_idx - 1;
            if !visited.contains(&new_idx)
                && !occupied.contains(&new_idx)
                && !queue.contains(&new_idx)
            {
                queue.push_back(new_idx);
            }
        }
        if curr_idx >= EXTENT {
            let new_idx = curr_idx - EXTENT;
            if !visited.contains(&new_idx)
                && !occupied.contains(&new_idx)
                && !queue.contains(&new_idx)
            {
                queue.push_back(new_idx);
            }
        };
        if curr_idx >= EXTENT * EXTENT {
            let new_idx = curr_idx - EXTENT * EXTENT;
            if !visited.contains(&new_idx)
                && !occupied.contains(&new_idx)
                && !queue.contains(&new_idx)
            {
                queue.push_back(new_idx);
            }
        };
    }
    println!("{}", total_surface_area);
}
