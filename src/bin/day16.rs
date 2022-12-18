use lazy_static::lazy_static;
use ndarray::Array3;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

struct Valve<'a> {
    name: &'a str,
    rate: usize,
    neighbours: Vec<&'a str>,
}

fn parse_input(input: &String) -> (HashMap<&str, usize>, Vec<Valve>) {
    lazy_static! {
        static ref VALVE_RE: Regex = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z][A-Z](?:, )?)+)").unwrap();
    }
    let mut valves = vec![];
    let captures_iter = VALVE_RE.captures_iter(&input[..]);
    for (i, valve_capture) in captures_iter.enumerate() {
        let rate = valve_capture
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let neighbours_str = valve_capture.get(3).unwrap().as_str();
        let neighbours = neighbours_str.split(", ").map(|x| x).collect();
        valves.push(Valve {
            name: valve_capture.get(1).unwrap().as_str(),
            rate,
            neighbours,
        });
    }
    // Sort valves by rate, useful for searching later
    valves.sort_by(|a, b| b.rate.cmp(&a.rate));
    let valve_map = valves
        .iter()
        .enumerate()
        .map(|(i, valve)| (valve.name, i))
        .collect::<HashMap<&str, usize>>();

    (valve_map, valves)
}

fn main() {
    // Init input reader
    let input = fs::read_to_string(r".\src\data\day16_1.txt").expect("Error opening file");
    // Parse input and find ID of start
    let (valve_map, valves) = parse_input(&input);
    let aa_idx = valve_map["AA"];

    // number of non trivial valves
    let m = valves.iter().filter(|v| v.rate > 0).count();
    let n = valves.len();
    let mm = 1 << m; //bitset for valves being open(0) / closed(1), 2^m

    let mut adjacency = vec![vec![0; 0]; n];
    for v in valves.iter() {
        let i = valve_map[v.name];
        for neighbour in v.neighbours.iter() {
            adjacency[i].push(valve_map[neighbour]);
        }
    }

    let mut opt = Array3::<usize>::zeros([30, n, mm]);
    // This loop works by iterating backwards through time. We start at all possible final
    // configurations. This allows us to recursively calculate 'what is the optimal value
    // of being in this configuration at a given time?'.
    //
    // At the final timestep, all possible configurations have the same value. We cannot gain
    // anything more once the problem is over.
    //
    // Going one time step back, we can enumerate the set of possible configurations from which
    // the next configuration could have been reached. This is
    // every config in which a valve was off and we turned it on, or every config in which
    // we were at a neighbouring valve and moved to this valve.
    //
    // Doing this iteratively we end up with an array opt[(time left, current valve, current state)]
    // which gives the maximum future return for each possible value of those variables. From there,
    // we just have to check the maximum future return of (30 mins left, "AA", all valves closed)
    // and we have the answer!
    for t in 1..30 {
        // loop over time steps
        for i in 0..n {
            // loop over each valve, calculating value function assuming I am
            // at this valve.
            let ii = 1 << i; // bit mask for this valve being opened
            for x in 0..mm {
                // loop over possible configurations
                let mut curr_val = opt[(t, i, x)];
                // is valve i closed in this config?
                if (ii & x) != 0 && t >= 2 {
                    // the possible return of turning valve i on at this timestep is the possible
                    // return of the config which is reached after having turned this
                    // valve on plus the return from turning this valve on
                    curr_val = curr_val.max(opt[(t - 1, i, x - ii)] + valves[i].rate * t as usize);
                }
                // I might gain more on this step from moving to an adjacent valve, can enumerate
                // over value function from the future frame to determine this (it might also
                // just not be possible to open a valve in this room in this state, so I have to
                // move).
                for &j in adjacency[i].iter() {
                    curr_val = curr_val.max(opt[(t - 1, j, x)]);
                }
                opt[(t, i, x)] = curr_val;
            }
        }
    }
    let res = opt[(29, aa_idx, mm - 1)];
    println!("{}", res);

    // part 2
    // enumerate all possible ways of dividing the set of valves into two
    // then solve the dp problem for me and my elephant with 26 minutes remaining from every
    // possible divison of the set

    // we can restrict the sets "my_valves" and "elephant_valves" by treating some of the valves
    // as already open (bitmask set to 0) in my initial state, and likewise for the elephant
    let mut max_res = 0;
    for i in 0..mm / 2 {
        let my_valves = i;
        let elephant_valves = mm - 1 - my_valves;
        let res = opt[(25, aa_idx, my_valves)] + opt[(25, aa_idx, elephant_valves)];
        if res > max_res {
            max_res = res;
        }
    }
    println!("{}", max_res);
}
