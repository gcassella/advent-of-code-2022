use std::{
    cell::RefCell,
    fs::File,
    io::{self, BufRead},
    rc::Rc,
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

#[derive(Debug)]
struct Segment {
    pos: (i32, i32),
    prev: Option<Rc<RefCell<Segment>>>,
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

/// Two physical observations are essential to my solution today.
///
/// The first is
/// that this problem exists in what I call the 'Dungeons and Dragons' metric,
/// where diagonal moves are the same distance as cardinal moves. That is,
///
/// d(dx, dy) = max(dx, dy)
///
/// This is also known as the Chebyshev distance. We only ever move the tail
/// of a segment if the Chebyshev distance between the head and the tail is
/// greater than 1.
///
/// The second observation is that the update rule for the tail can be
/// applied consistently for any separation by adding the displacement between
/// the head and the tail with the length along each axis clamped to +-1
///
/// With these in hand, this becomes a simple book-keeping exercise.
///
/// I got to do another new fun data structure this week: a mutable Linked List.
/// Being a simpler structure than the mutable Tree I used in day 7, I managed
/// this quite easily using the things I learned there about the interior
/// mutability pattern.
///
/// To be honest, I didn't need the linked list. I could have done this with
/// just a vector. But the linked list was cool, and I wrote it at 3am, so
/// that's good I guess.
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

    // We can model the lengths of rope with multiple segements (part 2) using
    // a linked list. I'm going to use the interior mutability pattern for this
    // because I need to let the LL be mutable. Each element in the list is an
    // encapsulaed smart pointer i.e. Rc<RefCell<Segment>>, and each
    // Segment holds another one of these pointers to its child segment
    // (except the 'tail' segment which has no child).

    // For Part 1 this is redundant overkill. We don't actually use the linked
    // list at all because I can just explicitly index the tail and head
    // variables to get the job done.

    let tail = Rc::new(RefCell::new(Segment {
        pos: (0, 0),
        prev: None,
    }));
    let head = Rc::new(RefCell::new(Segment {
        pos: (0, 0),
        prev: Some(Rc::clone(&tail)),
    }));

    let mut visited_positions: Vec<(i32, i32)> = vec![(0, 0)];

    for cmd in commands.iter() {
        for _ in 0..cmd.distance {
            match cmd.direction {
                Direction::Up => {
                    head.borrow_mut().pos.1 += 1 as i32;
                }
                Direction::Right => {
                    head.borrow_mut().pos.0 += 1 as i32;
                }
                Direction::Down => {
                    head.borrow_mut().pos.1 -= 1 as i32;
                }
                Direction::Left => {
                    head.borrow_mut().pos.0 -= 1 as i32;
                }
            }
            let dist = dndmetric(&head.borrow().pos, &tail.borrow().pos);
            if dist > 1 {
                // Use a let binding here so I don't have to immut borrow and
                // mut borrow tail simultaneously.
                let new_tail_pos = update_tail(&tail.borrow().pos, &head.borrow().pos);
                if !visited_positions.contains(&new_tail_pos) {
                    visited_positions.push(new_tail_pos);
                }
                tail.borrow_mut().pos = new_tail_pos;
            };
        }
    }

    println!("{}", visited_positions.len());

    // For part 2 the linked list becomes more useful, after each move I can
    // walk through the list and update the previous segment position of each
    // segment after it is moved

    // Initialize linked list with tail with no prev segment
    let mut curr_seg: Rc<RefCell<Segment>> = Rc::new(RefCell::new(Segment {
        pos: (0, 0),
        prev: None,
    }));
    let tail = Rc::clone(&curr_seg);
    for _ in 0..9 {
        curr_seg = Rc::new(RefCell::new(Segment {
            pos: (0, 0),
            prev: Some(Rc::clone(&curr_seg)),
        }));
    }
    // Store an immutable reference to the head of the rope
    let head = Rc::clone(&curr_seg);
    // Now we have our linked list in memory and a pointer to the start
    let mut visited_positions: Vec<(i32, i32)> = vec![(0, 0)];

    for cmd in commands.iter() {
        for _ in 0..cmd.distance {
            // update head just as before
            match cmd.direction {
                Direction::Up => {
                    head.borrow_mut().pos.1 += 1 as i32;
                }
                Direction::Right => {
                    head.borrow_mut().pos.0 += 1 as i32;
                }
                Direction::Down => {
                    head.borrow_mut().pos.1 -= 1 as i32;
                }
                Direction::Left => {
                    head.borrow_mut().pos.0 -= 1 as i32;
                }
            }
            // walk through list and move segments as appropriate
            let mut curr_seg = Rc::clone(&head);
            loop {
                // We need a new reference count for curr_seg here to avoid
                // some self-borrow-assign shenanigans
                let curr_seg_clone = Rc::clone(&curr_seg);
                match curr_seg_clone.borrow().prev.as_ref() {
                    Some(prev_seg) => {
                        let dist = dndmetric(&curr_seg.borrow().pos, &prev_seg.borrow().pos);

                        if dist > 1 {
                            // Use a let binding here so I don't have to immut borrow and
                            // mut borrow tail simultaneously.
                            let new_prev_pos =
                                update_tail(&prev_seg.borrow().pos, &curr_seg.borrow().pos);
                            prev_seg.borrow_mut().pos = new_prev_pos;
                        } else {
                            // We can break if any segment doesn't move, because
                            // it's children then won't update
                            break;
                        }
                    }
                    None => break,
                }
                // Now we're done with this clone we can use it to grab a ref
                // to the previous segment to take as the current segment for
                // the next iteration of the loop, and happily let the clone
                // drop out of scope. Everyone is happy.
                curr_seg = Rc::clone(curr_seg_clone.borrow().prev.as_ref().unwrap());
            }
            // check if we have a new pos for tail
            let tail_pos = tail.borrow().pos;
            if !visited_positions.contains(&tail_pos) {
                visited_positions.push(tail_pos);
            }
        }
    }

    println!("{}", visited_positions.len());
}
