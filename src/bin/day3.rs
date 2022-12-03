use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Backpack {
    compartment1: Vec<char>,
    compartment2: Vec<char>,
}

impl Backpack {
    pub fn from_str(line: &str) -> Backpack {
        let n = line.len();
        Backpack {
            compartment1: line[0..n / 2].chars().collect(),
            compartment2: line[n / 2..].chars().collect(),
        }
    }
}

/// Return index in `v` where `val` first appears. If `val` does not appear in
/// `v`, return None
fn bisect<T: std::cmp::PartialEq + std::cmp::PartialOrd>(v: Vec<T>, val: T) -> Option<usize> {
    let mut lo = 0;
    let mut hi = v.len() - 1;
    let mut mid: usize;
    while lo < hi {
        if val == v[lo] {
            return Some(lo);
        } else if val == v[hi] {
            return Some(hi);
        }

        mid = (lo + hi) / 2;

        if val == v[mid] {
            return Some(mid);
        } else if val < v[mid] {
            hi = mid
        } else {
            lo = mid + 1
        }
    }
    None
}

/// We can devise a O(n log(n)) solution to this problem using sorting and
/// bisection, rather than the O(n^2) solution of comparing every item
/// in the two compartments
fn main() {
    let priorities: HashMap<char, usize> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    let input = fs::read_to_string("day3_1.txt").expect("Error opening file");
    let mut accumulator = 0;
    for line in input.split("\r\n") {
        let mut bp = Backpack::from_str(line);
        // Sorting each compartment is O(N)
        bp.compartment1
            .sort_by(|a, b| priorities[a].cmp(&priorities[b]));
        bp.compartment2
            .sort_by(|a, b| priorities[a].cmp(&priorities[b]));
        // Now for each item in compartment 1, we can use bisection to find if
        // it also exists in compartment 2, incurring O(n logn)
        for item in bp.compartment1 {
            let val = priorities[&item];
            let prios = bp.compartment2.iter().map(|x| priorities[x]).collect();
            let comp2_pos = bisect(
                prios,
                val
            );
            match comp2_pos {
                Some(pos) => { 
                    accumulator += priorities[&bp.compartment2[pos]];
                    break;
                },
                None => continue
            }
        }
    }
    println!("{}", accumulator);
}
