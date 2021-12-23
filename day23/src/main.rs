use std::fs::File;
use std::io::prelude::*;
use trace::trace;

trace::init_depth_var!();

#[derive(Clone, Debug, PartialEq)]
struct Burrow {
    neighbors: Vec<Vec<(usize, u32)>>,
    amphipod: Vec<Option<u32>>,
    cost: u32,
}

impl Burrow {
    fn is_win(&self) -> bool {
        self.amphipod[7] == Some(1)
            && self.amphipod[8] == Some(1)
            && self.amphipod[9] == Some(10)
            && self.amphipod[10] == Some(10)
            && self.amphipod[11] == Some(100)
            && self.amphipod[12] == Some(100)
            && self.amphipod[13] == Some(1000)
            && self.amphipod[14] == Some(1000)
    }
}

type Input = Burrow;
type Part1Output = Option<u32>;
type Part2Output = usize;

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
    let lines = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();
    let mut amphipod = vec![
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    ];
    let line1 = lines[2].chars().filter(|c| *c != '#').collect::<Vec<_>>(); // neighbors 7 9 11 and 13
    amphipod[7] = Some(parse_letter(line1[0]));
    amphipod[9] = Some(parse_letter(line1[1]));
    amphipod[11] = Some(parse_letter(line1[2]));
    amphipod[13] = Some(parse_letter(line1[3]));
    let line2 = lines[3].chars().filter(|c| *c != '#').collect::<Vec<_>>(); // spaces 8 10 12 and 14
    amphipod[8] = Some(parse_letter(line2[0]));
    amphipod[10] = Some(parse_letter(line2[1]));
    amphipod[12] = Some(parse_letter(line2[2]));
    amphipod[14] = Some(parse_letter(line2[3]));

    return Ok(Burrow {
        neighbors: vec![
            vec![(1, 1)],                           // 0
            vec![(0, 1), (2, 2), (7, 2)],           // 1
            vec![(1, 1), (7, 2), (3, 2), (9, 2)],   // 2
            vec![(2, 2), (4, 2), (9, 2), (11, 2)],  // 3
            vec![(3, 2), (11, 2), (13, 2), (5, 2)], // 4
            vec![(4, 2), (13, 2), (6, 1)],          // 5
            vec![(5, 1)],                           // 6
            vec![(1, 2), (2, 2), (8, 1)],           // 7
            vec![(7, 1)],                           // 8
            vec![(2, 2), (3, 2), (10, 1)],          // 9
            vec![(9, 1)],                           // 10
            vec![(3, 2), (4, 2), (12, 1)],          // 11
            vec![(11, 1)],                          // 12
            vec![(4, 2), (5, 2), (14, 1)],          // 13
            vec![(13, 1)],                          // 14
        ],
        amphipod,
        cost: 0,
    });
}

fn parse_letter(l: char) -> u32 {
    match l {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Should not find this letter: {}", l),
    }
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    return Ok(best_path(input, 15000));
}

#[trace]
fn best_path(state: &Burrow, max_cost: u32) -> Option<u32> {
    if state.is_win() {
        return Some(state.cost);
    }
    let all_possible_next_moves = next_moves(state);
    all_possible_next_moves
        .iter()
        .filter(|m| m.cost < max_cost)
        .map(|m| best_path(m, max_cost))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .min()
}

#[trace]
fn next_moves(state: &Burrow) -> Vec<Burrow> {
    let mut moves: Vec<Burrow> = Vec::new();
    for (position, amphipod) in state
        .amphipod
        .iter()
        .enumerate()
        .filter(|(_, a)| a.is_some())
        .map(|(index, a)| (index, a.unwrap()))
    {
        let neighbors: Vec<(usize, u32)> = state.neighbors[position].clone();
        for &(neighbor, cost) in neighbors.iter() {
            if state.amphipod[neighbor].is_none() {
                let mut next_burrow = state.clone();
                next_burrow.amphipod[neighbor] = next_burrow.amphipod[position].clone();
                next_burrow.amphipod[position] = None;
                next_burrow.cost += cost * amphipod;
                moves.push(next_burrow);
            }
        }
    }
    return moves;
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    return Ok(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        Burrow {
            neighbors: vec![
                vec![(1, 1)],                           // 0
                vec![(0, 1), (2, 2), (7, 2)],           // 1
                vec![(1, 1), (7, 2), (3, 2), (9, 2)],   // 2
                vec![(2, 2), (4, 2), (9, 2), (11, 2)],  // 3
                vec![(3, 2), (11, 2), (13, 2), (5, 2)], // 4
                vec![(4, 2), (13, 2), (6, 1)],          // 5
                vec![(5, 1)],                           // 6
                vec![(1, 2), (2, 2), (8, 1)],           // 7
                vec![(7, 1)],                           // 8
                vec![(2, 2), (3, 2), (10, 1)],          // 9
                vec![(9, 1)],                           // 10
                vec![(3, 2), (4, 2), (12, 1)],          // 11
                vec![(11, 1)],                          // 12
                vec![(4, 2), (5, 2), (14, 1)],          // 13
                vec![(13, 1)],                          // 14
            ],
            amphipod: vec![
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(10),
                Some(1),
                Some(100),
                Some(1000),
                Some(10),
                Some(100),
                Some(1000),
                Some(1),
            ],
            cost: 0,
        }
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "#############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = Some(12521);
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 1;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
