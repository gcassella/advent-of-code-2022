use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug)]
struct Factory {
    ore_cost: usize,
    clay_cost: usize,
    obby_cost: (usize, usize),
    geod_cost: (usize, usize),
}

/// Assuming we create a geobot once a minute every minute for the remaining time, how many
/// more geodes can we get?
///
/// mr = curr_geobots * time_remaining + (curr_geobots+1)*(time_remaining-1)...
///
/// yes, i know i can figure out an analytical expression for this. no i can't be bothered to
/// do that after a bottle of mulled wine.
fn max_reward(time_remaining: usize, current_geobots: usize) -> usize {
    let mut sum = 0;
    for i in 0..time_remaining {
        sum += (current_geobots + i) * (time_remaining - i);
    }
    sum
}

/// I was a bit lazy with this, so my code is quite sloppy. If I wrote it again I would refactor
/// the build orders into an enum of robot types and a static lookup table of their costs
/// and the effect on the resources so I
/// could just issue all the build orders with one function, and have all the data in one place.
///
/// Building in release mode this is actually not too shabby on the runtime front, much better
/// than I initially expected considering the size of the state space! This required quite a lot
/// of pruning techniques that I'm not ashamed to admit I found on r/adventofcode. These are
///
/// 1) discard states that can't possibly exceed the best state seen so far even with an unphysical
///     amount of geode bot production
/// 2) avoid repeatedly entering the same state from different branches (although I feel like this
///     isn't possible, but I haven't thought about it long enough)
/// 3) never build more robots than the maximum possible required production of that resource per
///     minute
/// 4) order the insertion of states into the queue so that (1) is likely to prune a lot of useless
///     states where we do nothing for long periods of time.
/// 5) if a geobot can be built, assume this is optimal and do not queue any sibling states
fn main() {
    lazy_static! {
        static ref ROBOT_RE: Regex = Regex::new(r"Blueprint (\d+): .+(\d+) ore\..+(\d+) ore\..+(\d+) ore and (\d+) clay\..+(\d+) ore and (\d+) obsidian\.").unwrap();
    }
    let input = fs::read_to_string(r".\src\data\day19_1.txt").expect("");
    let factories: Vec<Factory> = ROBOT_RE
        .captures_iter(&input)
        .map(|x| Factory {
            ore_cost: x.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            clay_cost: x.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            obby_cost: (
                x.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                x.get(5).unwrap().as_str().parse::<usize>().unwrap(),
            ),
            geod_cost: (
                x.get(6).unwrap().as_str().parse::<usize>().unwrap(),
                x.get(7).unwrap().as_str().parse::<usize>().unwrap(),
            ),
        })
        .collect();

    type State = (usize, usize, usize, usize, usize, usize, usize, usize);
    // for each factory we can do a simple dfs over states to find their max reward
    let init_state: State = (0, 1, 0, 0, 0, 0, 0, 0);
    let mut factory_values: Vec<usize> = vec![];
    // part 1
    // for factory in factories.iter() {
    // part 2
    for factory in factories.iter().take(3) {
        println!("{:?}", factory);

        let max_ore_cost = factory
            .ore_cost
            .max(factory.clay_cost)
            .max(factory.obby_cost.0)
            .max(factory.geod_cost.0);

        // state = time, orebots, claybots, obbybots, geobots, ore, clay, obby
        // state_rewards maps state to number of geodes
        let mut state_rewards: HashMap<State, usize> = HashMap::new();
        state_rewards.insert(init_state, 0);

        //dfs
        let mut queue: VecDeque<State> = VecDeque::new();
        queue.push_front(init_state);
        let mut maxval = 0;
        while queue.len() > 0 {
            let state = queue.pop_front().unwrap();
            //part 1
            // if state.0 == 24 {
            // part 2
            if state.0 == 32 {
                // time is up, or we've been here before, do not advance this state
                continue;
            }
            let curr_reward = state_rewards[&state];

            // part 1
            // let future_max_reward = curr_reward + max_reward(24 - state.0, state.4);
            // part 2
            let future_max_reward = curr_reward + max_reward(32 - state.0, state.4);

            if future_max_reward < maxval {
                // we can't possibly beat the current best state with this, skip!
                continue;
            }

            let next_reward = state.4 + state_rewards[&state];
            if next_reward > maxval {
                maxval = next_reward;
            }

            // build geobot
            if state.5 >= factory.geod_cost.0 && state.7 >= factory.geod_cost.1 {
                let next_state = (
                    state.0 + 1, // increment time
                    state.1,     // orebots stays same
                    state.2,     // claybots stays same
                    state.3,     // obbybots stays same
                    state.4 + 1, // increment geobots
                    state.5 - factory.geod_cost.0 + state.1,
                    state.6 + state.2,
                    state.7 - factory.geod_cost.1 + state.3,
                );

                if !state_rewards.contains_key(&next_state) {
                    queue.push_front(next_state);
                    state_rewards.insert(next_state, next_reward);
                    // if we built a geobot, consider this an optimal move and dont push any other
                    // choices onto the search
                    continue;
                }
            }
            let next_state = (
                state.0 + 1, // increment time
                state.1,     // orebots stays same
                state.2,     // claybots stays same
                state.3,     // obbybots stays same
                state.4,     // geobots stays same
                state.5 + state.1,
                state.6 + state.2,
                state.7 + state.3,
            );
            if !state_rewards.contains_key(&next_state) {
                queue.push_front(next_state);
                state_rewards.insert(next_state, next_reward);
            }

            // build orebot
            if state.5 >= factory.ore_cost && state.1 < max_ore_cost {
                let next_state = (
                    state.0 + 1,                          // increment time
                    state.1 + 1,                          // increment orebots
                    state.2,                              // claybots stays same
                    state.3,                              // obbybots stays same
                    state.4,                              // geobots stays same
                    state.5 - factory.ore_cost + state.1, // spend ore and gather ore
                    state.6 + state.2,
                    state.7 + state.3,
                );
                if !state_rewards.contains_key(&next_state) {
                    queue.push_front(next_state);
                    state_rewards.insert(next_state, next_reward);
                }
            }
            // build claybot, unless we already farm more than max clay cost per minute
            if state.5 >= factory.clay_cost && state.2 < factory.obby_cost.1 {
                let next_state = (
                    state.0 + 1, // increment time
                    state.1,     // orebots stays same
                    state.2 + 1, // increment claybots
                    state.3,     // obbybots stays same
                    state.4,     // geobots stays same
                    state.5 - factory.clay_cost + state.1,
                    state.6 + state.2,
                    state.7 + state.3,
                );
                if !state_rewards.contains_key(&next_state) {
                    queue.push_front(next_state);
                    state_rewards.insert(next_state, next_reward);
                }
            }
            // build obbybot, unless we already farm more than max obby cost per minute
            if state.5 >= factory.obby_cost.0
                && state.6 >= factory.obby_cost.1
                && state.3 < factory.geod_cost.1
            {
                let next_state = (
                    state.0 + 1, // increment time
                    state.1,     // orebots stays same
                    state.2,     // claybots stays same
                    state.3 + 1, // increment obbybots
                    state.4,     // geobots stays same
                    state.5 - factory.obby_cost.0 + state.1,
                    state.6 - factory.obby_cost.1 + state.2,
                    state.7 + state.3,
                );
                if !state_rewards.contains_key(&next_state) {
                    queue.push_front(next_state);
                    state_rewards.insert(next_state, next_reward);
                }
            }
        }

        // store max value obtains by factory
        println!("{}", maxval);
        factory_values.push(maxval);
    }

    // part 1
    // println!("{}", factory_values.iter().enumerate().map(|(i, x)| (i+1)*x).fold(0, |a, b| a+b));
    // part 2
    println!("{}", factory_values.iter().fold(1, |a, b| a * b));
}
