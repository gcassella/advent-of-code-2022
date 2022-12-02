use std::collections::HashMap;
use std::fs;

fn rps_score(enemy_move: usize, player_move: usize) -> usize {
    if enemy_move == player_move {
        return 3;
    } else {
        if enemy_move == 1 {
            // rock
            if player_move == 2 {
                // vs paper
                return 6;
            } else if player_move == 3 {
                // vs scissors
                return 0;
            }
        } else if enemy_move == 2 {
            // paper
            if player_move == 1 {
                // vs rock
                return 0;
            } else if player_move == 3 {
                // vs scissors
                return 6;
            }
        } else if enemy_move == 3 {
            // scissors
            if player_move == 1 {
                // vs rock
                return 6;
            } else if player_move == 2 {
                // vs paper
                return 0;
            }
        }
    }
    0
}

fn parse_input(input: String) -> Result<(Vec<usize>, Vec<usize>), String> {
    let moves: HashMap<&str, usize> =
        HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    let mut enemy_moves = vec![];
    let mut player_moves = vec![];
    for v in input
        .split("\r\n") // Sorry, linux users
        .map(|x| x.split(' ').collect::<Vec<&str>>())
    {
        enemy_moves.push(*moves.get(v[0]).expect("Badly formatted move in input"));
        player_moves.push(*moves.get(v[1]).expect("Badly formatted move in input"));
    }
    Ok((enemy_moves, player_moves))
}

/// Calculate our score under the false assumption that the strategy guide
/// means X = rock, Y = paper, Z = scissors
fn calculate_score(enemy_moves: &Vec<usize>, player_moves: &Vec<usize>) -> usize {
    let mut score = 0;
    for round in enemy_moves.iter().zip(player_moves.iter()) {
        score += round.1;
        score += rps_score(*round.0, *round.1);
    }
    score
}

/// Calculate our score under the correct strategy
/// X = lose, Y = draw, Z = win
fn calculate_strategy_score(enemy_moves: &Vec<usize>, player_moves: &Vec<usize>) -> usize {
    let strategy = HashMap::from([
        ((1, 1), 0 + 3), // lose + scissors
        ((1, 2), 3 + 1), // draw + rock
        ((1, 3), 6 + 2), // win + paper
        ((2, 1), 0 + 1), // lose + rock
        ((2, 2), 3 + 2), // draw + paper
        ((2, 3), 6 + 3), // win + scissors
        ((3, 1), 0 + 2), // lose + paper
        ((3, 2), 3 + 3), // draw + scissors
        ((3, 3), 6 + 1), // win + rock
    ]);
    let mut score = 0;
    for round in enemy_moves.iter().zip(player_moves.iter()) {
        score += strategy.get(&(*round.0, *round.1)).unwrap();
    }
    score
}

fn main() {
    let input = fs::read_to_string("day2_1.txt").expect("Error opening file");
    let parsed_in = parse_input(input).unwrap();
    println!("{:?}", calculate_score(&parsed_in.0, &parsed_in.1));
    println!("{:?}", calculate_strategy_score(&parsed_in.0, &parsed_in.1));
}
