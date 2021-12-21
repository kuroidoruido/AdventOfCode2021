use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::RwLock;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Player {
    position: u128,
    score: u128,
}

impl std::str::FromStr for Player {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Player 1 starting position: 3
        let (_, initial_position) = s.split_once("position: ").unwrap();
        Ok(Player {
            position: initial_position.parse::<u128>().unwrap(),
            score: 0,
        })
    }
}

type Players = Vec<Player>;

type Input = Players;
type Part1Output = u128;
type Part2Output = u128;

fn main() -> std::io::Result<()> {
    let input1 = read_input("input1.txt").expect("An error occurred when reading input1.txt");
    let data1 = parse_data(input1).expect("An error occurred when parsing input1.txt");

    println!("Part 1: {:?}", part1(&data1));
    println!("--------------------------------------------------");
    println!("Part 2: {:?}", part2(&data1));
    Ok(())
}

fn read_input(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn parse_data(input: String) -> Result<Input, String> {
    let lines: Vec<Player> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| file_fragment.parse::<Player>().unwrap())
        .collect();
    return Ok(lines);
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    let mut players = input.clone();
    let mut current_player: usize = 0;
    let mut dice_rolled: u128 = 0;
    let mut last_dice: u128 = 0;
    loop {
        let (first, second, third) = next_deterministic_dices(last_dice);
        last_dice = third;
        dice_rolled += 3;

        let player_position =
            (players[current_player].position + first + second + third - 1) % 10 + 1;
        players[current_player].position = player_position;
        players[current_player].score += player_position;

        if players[current_player].score >= 1000 {
            break;
        }
        current_player = (current_player + 1) % players.len();
    }
    let loser_score = players.iter().map(|p| p.score).min().unwrap();
    return Ok(loser_score * dice_rolled);
}

fn next_deterministic_dices(last_dice: u128) -> (u128, u128, u128) {
    (
        (last_dice % 1000) + 1,
        (last_dice + 1 % 1000) + 1,
        (last_dice + 2 % 1000) + 1,
    )
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let initial_player = 0;
    let wins = quantum_run(input.clone(), initial_player);
    return Ok(*wins.iter().max().unwrap());
}

lazy_static! {
    static ref CACHE: RwLock<HashMap<(Players, usize), Vec<u128>>> = RwLock::new(HashMap::new());
}

fn quantum_run(players: Players, current_player: usize) -> Vec<u128> {
    {
        let cache = CACHE.read().unwrap();
        if let Some(previous_res) = cache.get(&(players.clone(), current_player)) {
            return previous_res.clone();
        }
    }
    let dice_sets = next_quantum_dices();
    let mut player_wins: Vec<u128> = vec![0; players.len()];
    let next_player = (current_player + 1) % players.len();
    for (first, second, third) in dice_sets.iter() {
        let mut universe_player = players.clone();
        let player_position =
            (universe_player[current_player].position + first + second + third - 1) % 10 + 1;
        universe_player[current_player].position = player_position;
        universe_player[current_player].score += player_position;
        if universe_player[current_player].score >= 21 {
            player_wins[current_player] += 1;
        } else {
            let scores = quantum_run(universe_player, next_player);
            scores.iter().enumerate().for_each(|(index, score)| {
                player_wins[index] += score;
            });
        }
    }
    let mut cache = CACHE.write().unwrap();
    cache.insert((players.clone(), current_player), player_wins.clone());
    return player_wins;
}

fn next_quantum_dices() -> Vec<(u128, u128, u128)> {
    let mut dices: Vec<(u128, u128, u128)> = Vec::new();
    for x in 1..4 {
        for y in 1..4 {
            for z in 1..4 {
                dices.push((x, y, z));
            }
        }
    }
    return dices;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        vec![
            Player {
                position: 4,
                score: 0,
            },
            Player {
                position: 8,
                score: 0,
            },
        ]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "Player 1 starting position: 4
        Player 2 starting position: 8
        ";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = 739785;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 444356092776315;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
