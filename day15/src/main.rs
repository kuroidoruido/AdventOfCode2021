use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Input = Vec<Vec<u32>>;
type Part1Output = u32;
type Part2Output = u32;

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
    let lines: Input = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| {
            file_fragment
                .split("")
                .map(|file_fragment| file_fragment.trim())
                .filter(|file_fragment| !file_fragment.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    return Ok(lines);
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    let path = find_lowest_risk_path(input, (0, 0));
    return Ok(path.unwrap().iter().sum::<u32>() - input[0][0]);
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let real_map = compute_real_map(input, 5);
    let path = find_lowest_risk_path(&real_map, (0, 0));
    return Ok(path.unwrap().iter().sum::<u32>() - input[0][0]);
}

type Node = (u32, (usize, usize), u32);

fn find_lowest_risk_path(data: &Vec<Vec<u32>>, start_position: (usize, usize)) -> Option<Vec<u32>> {
    fn get_lowest_fscore(open_set: &Vec<Node>, f_score: &HashMap<Node, u32>) -> Node {
        *open_set
            .iter()
            .min_by(|x, y| f_score.get(x).unwrap().cmp(f_score.get(y).unwrap()))
            .unwrap()
    }
    fn reconstruct_path(came_from: &HashMap<Node, Node>, last: Node) -> Vec<u32> {
        let mut path: Vec<u32> = vec![last.0];
        let mut current = last;
        while let Some(from) = came_from.get(&current) {
            current = *from;
            path.push(current.0);
        }
        return path.iter().rev().cloned().collect();
    }

    let end_position: (usize, usize) = (data[0].len() - 1, data.len() - 1);
    let matrix: Vec<Vec<Node>> = data
        .iter()
        .enumerate()
        .map(|(j, row)| {
            row.iter()
                .enumerate()
                .map(|(i, cell)| (*cell, (i, j), manhattan(&(i, j), &end_position)))
                .collect::<Vec<Node>>()
        })
        .collect();
    // A*
    let start = matrix[start_position.1][start_position.0];
    let goal = matrix[end_position.1][end_position.0];
    let mut open_set: Vec<Node> = vec![start];
    let mut came_from: HashMap<Node, Node> = HashMap::new();
    let mut g_score: HashMap<Node, u32> = HashMap::new();
    g_score.insert(start, 0);
    let mut f_score: HashMap<Node, u32> = HashMap::new();
    f_score.insert(start, start.2);

    while !open_set.is_empty() {
        let current = get_lowest_fscore(&open_set, &f_score);
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }
        open_set = open_set
            .iter()
            .filter(|node| **node != current)
            .cloned()
            .collect();
        let neighbors_position = get_adjacents_position(&matrix, current.1 .0, current.1 .1);
        let neighbors: Vec<Node> = neighbors_position
            .iter()
            .map(|(x, y)| matrix[*y][*x])
            .collect();
        for neighbor in neighbors {
            let tentative_g_score = g_score.get(&current).unwrap() + current.0;
            if tentative_g_score < *g_score.get(&neighbor).or(Some(&u32::MAX)).unwrap() {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + neighbor.2);
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }
    return None;
}

fn manhattan(start_position: &(usize, usize), end_position: &(usize, usize)) -> u32 {
    let start_x: i32 = start_position.0.try_into().unwrap();
    let start_y: i32 = start_position.1.try_into().unwrap();
    let end_x: i32 = end_position.0.try_into().unwrap();
    let end_y: i32 = end_position.1.try_into().unwrap();
    ((end_x - start_x).abs() + (end_y - start_y).abs())
        .try_into()
        .unwrap()
}

fn get_adjacents_position<T>(data: &Vec<Vec<T>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let max_y: isize = (data.len() - 1)
        .try_into()
        .expect("should be able to convert max_y to isize");
    let max_x: isize = (data[0].len() - 1)
        .try_into()
        .expect("should be able to convert max_x to isize");
    let x: isize = x.try_into().expect("should be able to convert x to isize");
    let y: isize = y.try_into().expect("should be able to convert y to isize");
    let adjacent_positions: Vec<(isize, isize)> =
        vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];
    adjacent_positions
        .iter()
        .filter(|(ax, ay)| *ax >= 0 && *ay >= 0 && *ax <= max_x && *ay <= max_y)
        .map(|(ax, ay)| {
            let ax: usize = usize::try_from(*ax).unwrap();
            let ay: usize = usize::try_from(*ay).unwrap();
            (ax, ay)
        })
        .collect()
}

fn compute_real_map(input: &Vec<Vec<u32>>, multiplicator: usize) -> Vec<Vec<u32>> {
    let input_i = input[0].len();
    let input_j = input.len();
    let map_i = input_i * multiplicator;
    let map_j = input_j * multiplicator;
    let mut map: Vec<Vec<u32>> = Vec::with_capacity(map_j);

    for _ in 0..map_j {
        map.push(vec![0; map_i]);
    }

    for j in 0..input.len() {
        for i in 0..input[0].len() {
            for increment in 0..multiplicator {
                let inc_u32: u32 = increment.try_into().unwrap();
                let new_val = input[j][i] + inc_u32;
                map[j][i + (increment * input_i)] = if new_val > 9 { new_val - 9 } else { new_val };
            }
        }
    }
    for j in 0..input.len() {
        for i in 0..map[0].len() {
            for increment in 0..multiplicator {
                let inc_u32: u32 = increment.try_into().unwrap();
                let new_val = map[j][i] + inc_u32;
                map[j + (increment * input_j)][i] = if new_val > 9 { new_val - 9 } else { new_val };
            }
        }
    }
    return map;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]
    }

    fn sample_real_map() -> Input {
        vec![
            vec![
                1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 2, 2, 7, 4, 8, 6, 2, 8, 5, 3, 3, 3, 8, 5, 9, 7, 3, 9,
                6, 4, 4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6,
            ],
            vec![
                1, 3, 8, 1, 3, 7, 3, 6, 7, 2, 2, 4, 9, 2, 4, 8, 4, 7, 8, 3, 3, 5, 1, 3, 5, 9, 5, 8,
                9, 4, 4, 6, 2, 4, 6, 1, 6, 9, 1, 5, 5, 7, 3, 5, 7, 2, 7, 1, 2, 6,
            ],
            vec![
                2, 1, 3, 6, 5, 1, 1, 3, 2, 8, 3, 2, 4, 7, 6, 2, 2, 4, 3, 9, 4, 3, 5, 8, 7, 3, 3, 5,
                4, 1, 5, 4, 6, 9, 8, 4, 4, 6, 5, 2, 6, 5, 7, 1, 9, 5, 5, 7, 6, 3,
            ],
            vec![
                3, 6, 9, 4, 9, 3, 1, 5, 6, 9, 4, 7, 1, 5, 1, 4, 2, 6, 7, 1, 5, 8, 2, 6, 2, 5, 3, 7,
                8, 2, 6, 9, 3, 7, 3, 6, 4, 8, 9, 3, 7, 1, 4, 8, 4, 7, 5, 9, 1, 4,
            ],
            vec![
                7, 4, 6, 3, 4, 1, 7, 1, 1, 1, 8, 5, 7, 4, 5, 2, 8, 2, 2, 2, 9, 6, 8, 5, 6, 3, 9, 3,
                3, 3, 1, 7, 9, 6, 7, 4, 1, 4, 4, 4, 2, 8, 1, 7, 8, 5, 2, 5, 5, 5,
            ],
            vec![
                1, 3, 1, 9, 1, 2, 8, 1, 3, 7, 2, 4, 2, 1, 2, 3, 9, 2, 4, 8, 3, 5, 3, 2, 3, 4, 1, 3,
                5, 9, 4, 6, 4, 3, 4, 5, 2, 4, 6, 1, 5, 7, 5, 4, 5, 6, 3, 5, 7, 2,
            ],
            vec![
                1, 3, 5, 9, 9, 1, 2, 4, 2, 1, 2, 4, 6, 1, 1, 2, 3, 5, 3, 2, 3, 5, 7, 2, 2, 3, 4, 6,
                4, 3, 4, 6, 8, 3, 3, 4, 5, 7, 5, 4, 5, 7, 9, 4, 4, 5, 6, 8, 6, 5,
            ],
            vec![
                3, 1, 2, 5, 4, 2, 1, 6, 3, 9, 4, 2, 3, 6, 5, 3, 2, 7, 4, 1, 5, 3, 4, 7, 6, 4, 3, 8,
                5, 2, 6, 4, 5, 8, 7, 5, 4, 9, 6, 3, 7, 5, 6, 9, 8, 6, 5, 1, 7, 4,
            ],
            vec![
                1, 2, 9, 3, 1, 3, 8, 5, 2, 1, 2, 3, 1, 4, 2, 4, 9, 6, 3, 2, 3, 4, 2, 5, 3, 5, 1, 7,
                4, 3, 4, 5, 3, 6, 4, 6, 2, 8, 5, 4, 5, 6, 4, 7, 5, 7, 3, 9, 6, 5,
            ],
            vec![
                2, 3, 1, 1, 9, 4, 4, 5, 8, 1, 3, 4, 2, 2, 1, 5, 5, 6, 9, 2, 4, 5, 3, 3, 2, 6, 6, 7,
                1, 3, 5, 6, 4, 4, 3, 7, 7, 8, 2, 4, 6, 7, 5, 5, 4, 8, 8, 9, 3, 5,
            ],
            vec![
                2, 2, 7, 4, 8, 6, 2, 8, 5, 3, 3, 3, 8, 5, 9, 7, 3, 9, 6, 4, 4, 4, 9, 6, 1, 8, 4, 1,
                7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6, 6, 6, 2, 8, 3, 1, 6, 3, 9, 7,
            ],
            vec![
                2, 4, 9, 2, 4, 8, 4, 7, 8, 3, 3, 5, 1, 3, 5, 9, 5, 8, 9, 4, 4, 6, 2, 4, 6, 1, 6, 9,
                1, 5, 5, 7, 3, 5, 7, 2, 7, 1, 2, 6, 6, 8, 4, 6, 8, 3, 8, 2, 3, 7,
            ],
            vec![
                3, 2, 4, 7, 6, 2, 2, 4, 3, 9, 4, 3, 5, 8, 7, 3, 3, 5, 4, 1, 5, 4, 6, 9, 8, 4, 4, 6,
                5, 2, 6, 5, 7, 1, 9, 5, 5, 7, 6, 3, 7, 6, 8, 2, 1, 6, 6, 8, 7, 4,
            ],
            vec![
                4, 7, 1, 5, 1, 4, 2, 6, 7, 1, 5, 8, 2, 6, 2, 5, 3, 7, 8, 2, 6, 9, 3, 7, 3, 6, 4, 8,
                9, 3, 7, 1, 4, 8, 4, 7, 5, 9, 1, 4, 8, 2, 5, 9, 5, 8, 6, 1, 2, 5,
            ],
            vec![
                8, 5, 7, 4, 5, 2, 8, 2, 2, 2, 9, 6, 8, 5, 6, 3, 9, 3, 3, 3, 1, 7, 9, 6, 7, 4, 1, 4,
                4, 4, 2, 8, 1, 7, 8, 5, 2, 5, 5, 5, 3, 9, 2, 8, 9, 6, 3, 6, 6, 6,
            ],
            vec![
                2, 4, 2, 1, 2, 3, 9, 2, 4, 8, 3, 5, 3, 2, 3, 4, 1, 3, 5, 9, 4, 6, 4, 3, 4, 5, 2, 4,
                6, 1, 5, 7, 5, 4, 5, 6, 3, 5, 7, 2, 6, 8, 6, 5, 6, 7, 4, 6, 8, 3,
            ],
            vec![
                2, 4, 6, 1, 1, 2, 3, 5, 3, 2, 3, 5, 7, 2, 2, 3, 4, 6, 4, 3, 4, 6, 8, 3, 3, 4, 5, 7,
                5, 4, 5, 7, 9, 4, 4, 5, 6, 8, 6, 5, 6, 8, 1, 5, 5, 6, 7, 9, 7, 6,
            ],
            vec![
                4, 2, 3, 6, 5, 3, 2, 7, 4, 1, 5, 3, 4, 7, 6, 4, 3, 8, 5, 2, 6, 4, 5, 8, 7, 5, 4, 9,
                6, 3, 7, 5, 6, 9, 8, 6, 5, 1, 7, 4, 8, 6, 7, 1, 9, 7, 6, 2, 8, 5,
            ],
            vec![
                2, 3, 1, 4, 2, 4, 9, 6, 3, 2, 3, 4, 2, 5, 3, 5, 1, 7, 4, 3, 4, 5, 3, 6, 4, 6, 2, 8,
                5, 4, 5, 6, 4, 7, 5, 7, 3, 9, 6, 5, 6, 7, 5, 8, 6, 8, 4, 1, 7, 6,
            ],
            vec![
                3, 4, 2, 2, 1, 5, 5, 6, 9, 2, 4, 5, 3, 3, 2, 6, 6, 7, 1, 3, 5, 6, 4, 4, 3, 7, 7, 8,
                2, 4, 6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6,
            ],
            vec![
                3, 3, 8, 5, 9, 7, 3, 9, 6, 4, 4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2,
                8, 6, 6, 6, 2, 8, 3, 1, 6, 3, 9, 7, 7, 7, 3, 9, 4, 2, 7, 4, 1, 8,
            ],
            vec![
                3, 5, 1, 3, 5, 9, 5, 8, 9, 4, 4, 6, 2, 4, 6, 1, 6, 9, 1, 5, 5, 7, 3, 5, 7, 2, 7, 1,
                2, 6, 6, 8, 4, 6, 8, 3, 8, 2, 3, 7, 7, 9, 5, 7, 9, 4, 9, 3, 4, 8,
            ],
            vec![
                4, 3, 5, 8, 7, 3, 3, 5, 4, 1, 5, 4, 6, 9, 8, 4, 4, 6, 5, 2, 6, 5, 7, 1, 9, 5, 5, 7,
                6, 3, 7, 6, 8, 2, 1, 6, 6, 8, 7, 4, 8, 7, 9, 3, 2, 7, 7, 9, 8, 5,
            ],
            vec![
                5, 8, 2, 6, 2, 5, 3, 7, 8, 2, 6, 9, 3, 7, 3, 6, 4, 8, 9, 3, 7, 1, 4, 8, 4, 7, 5, 9,
                1, 4, 8, 2, 5, 9, 5, 8, 6, 1, 2, 5, 9, 3, 6, 1, 6, 9, 7, 2, 3, 6,
            ],
            vec![
                9, 6, 8, 5, 6, 3, 9, 3, 3, 3, 1, 7, 9, 6, 7, 4, 1, 4, 4, 4, 2, 8, 1, 7, 8, 5, 2, 5,
                5, 5, 3, 9, 2, 8, 9, 6, 3, 6, 6, 6, 4, 1, 3, 9, 1, 7, 4, 7, 7, 7,
            ],
            vec![
                3, 5, 3, 2, 3, 4, 1, 3, 5, 9, 4, 6, 4, 3, 4, 5, 2, 4, 6, 1, 5, 7, 5, 4, 5, 6, 3, 5,
                7, 2, 6, 8, 6, 5, 6, 7, 4, 6, 8, 3, 7, 9, 7, 6, 7, 8, 5, 7, 9, 4,
            ],
            vec![
                3, 5, 7, 2, 2, 3, 4, 6, 4, 3, 4, 6, 8, 3, 3, 4, 5, 7, 5, 4, 5, 7, 9, 4, 4, 5, 6, 8,
                6, 5, 6, 8, 1, 5, 5, 6, 7, 9, 7, 6, 7, 9, 2, 6, 6, 7, 8, 1, 8, 7,
            ],
            vec![
                5, 3, 4, 7, 6, 4, 3, 8, 5, 2, 6, 4, 5, 8, 7, 5, 4, 9, 6, 3, 7, 5, 6, 9, 8, 6, 5, 1,
                7, 4, 8, 6, 7, 1, 9, 7, 6, 2, 8, 5, 9, 7, 8, 2, 1, 8, 7, 3, 9, 6,
            ],
            vec![
                3, 4, 2, 5, 3, 5, 1, 7, 4, 3, 4, 5, 3, 6, 4, 6, 2, 8, 5, 4, 5, 6, 4, 7, 5, 7, 3, 9,
                6, 5, 6, 7, 5, 8, 6, 8, 4, 1, 7, 6, 7, 8, 6, 9, 7, 9, 5, 2, 8, 7,
            ],
            vec![
                4, 5, 3, 3, 2, 6, 6, 7, 1, 3, 5, 6, 4, 4, 3, 7, 7, 8, 2, 4, 6, 7, 5, 5, 4, 8, 8, 9,
                3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6, 8, 9, 7, 7, 6, 1, 1, 2, 5, 7,
            ],
            vec![
                4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6, 6, 6, 2, 8, 3, 1, 6, 3,
                9, 7, 7, 7, 3, 9, 4, 2, 7, 4, 1, 8, 8, 8, 4, 1, 5, 3, 8, 5, 2, 9,
            ],
            vec![
                4, 6, 2, 4, 6, 1, 6, 9, 1, 5, 5, 7, 3, 5, 7, 2, 7, 1, 2, 6, 6, 8, 4, 6, 8, 3, 8, 2,
                3, 7, 7, 9, 5, 7, 9, 4, 9, 3, 4, 8, 8, 1, 6, 8, 1, 5, 1, 4, 5, 9,
            ],
            vec![
                5, 4, 6, 9, 8, 4, 4, 6, 5, 2, 6, 5, 7, 1, 9, 5, 5, 7, 6, 3, 7, 6, 8, 2, 1, 6, 6, 8,
                7, 4, 8, 7, 9, 3, 2, 7, 7, 9, 8, 5, 9, 8, 1, 4, 3, 8, 8, 1, 9, 6,
            ],
            vec![
                6, 9, 3, 7, 3, 6, 4, 8, 9, 3, 7, 1, 4, 8, 4, 7, 5, 9, 1, 4, 8, 2, 5, 9, 5, 8, 6, 1,
                2, 5, 9, 3, 6, 1, 6, 9, 7, 2, 3, 6, 1, 4, 7, 2, 7, 1, 8, 3, 4, 7,
            ],
            vec![
                1, 7, 9, 6, 7, 4, 1, 4, 4, 4, 2, 8, 1, 7, 8, 5, 2, 5, 5, 5, 3, 9, 2, 8, 9, 6, 3, 6,
                6, 6, 4, 1, 3, 9, 1, 7, 4, 7, 7, 7, 5, 2, 4, 1, 2, 8, 5, 8, 8, 8,
            ],
            vec![
                4, 6, 4, 3, 4, 5, 2, 4, 6, 1, 5, 7, 5, 4, 5, 6, 3, 5, 7, 2, 6, 8, 6, 5, 6, 7, 4, 6,
                8, 3, 7, 9, 7, 6, 7, 8, 5, 7, 9, 4, 8, 1, 8, 7, 8, 9, 6, 8, 1, 5,
            ],
            vec![
                4, 6, 8, 3, 3, 4, 5, 7, 5, 4, 5, 7, 9, 4, 4, 5, 6, 8, 6, 5, 6, 8, 1, 5, 5, 6, 7, 9,
                7, 6, 7, 9, 2, 6, 6, 7, 8, 1, 8, 7, 8, 1, 3, 7, 7, 8, 9, 2, 9, 8,
            ],
            vec![
                6, 4, 5, 8, 7, 5, 4, 9, 6, 3, 7, 5, 6, 9, 8, 6, 5, 1, 7, 4, 8, 6, 7, 1, 9, 7, 6, 2,
                8, 5, 9, 7, 8, 2, 1, 8, 7, 3, 9, 6, 1, 8, 9, 3, 2, 9, 8, 4, 1, 7,
            ],
            vec![
                4, 5, 3, 6, 4, 6, 2, 8, 5, 4, 5, 6, 4, 7, 5, 7, 3, 9, 6, 5, 6, 7, 5, 8, 6, 8, 4, 1,
                7, 6, 7, 8, 6, 9, 7, 9, 5, 2, 8, 7, 8, 9, 7, 1, 8, 1, 6, 3, 9, 8,
            ],
            vec![
                5, 6, 4, 4, 3, 7, 7, 8, 2, 4, 6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1,
                4, 6, 8, 9, 7, 7, 6, 1, 1, 2, 5, 7, 9, 1, 8, 8, 7, 2, 2, 3, 6, 8,
            ],
            vec![
                5, 5, 1, 7, 2, 9, 5, 2, 8, 6, 6, 6, 2, 8, 3, 1, 6, 3, 9, 7, 7, 7, 3, 9, 4, 2, 7, 4,
                1, 8, 8, 8, 4, 1, 5, 3, 8, 5, 2, 9, 9, 9, 5, 2, 6, 4, 9, 6, 3, 1,
            ],
            vec![
                5, 7, 3, 5, 7, 2, 7, 1, 2, 6, 6, 8, 4, 6, 8, 3, 8, 2, 3, 7, 7, 9, 5, 7, 9, 4, 9, 3,
                4, 8, 8, 1, 6, 8, 1, 5, 1, 4, 5, 9, 9, 2, 7, 9, 2, 6, 2, 5, 6, 1,
            ],
            vec![
                6, 5, 7, 1, 9, 5, 5, 7, 6, 3, 7, 6, 8, 2, 1, 6, 6, 8, 7, 4, 8, 7, 9, 3, 2, 7, 7, 9,
                8, 5, 9, 8, 1, 4, 3, 8, 8, 1, 9, 6, 1, 9, 2, 5, 4, 9, 9, 2, 1, 7,
            ],
            vec![
                7, 1, 4, 8, 4, 7, 5, 9, 1, 4, 8, 2, 5, 9, 5, 8, 6, 1, 2, 5, 9, 3, 6, 1, 6, 9, 7, 2,
                3, 6, 1, 4, 7, 2, 7, 1, 8, 3, 4, 7, 2, 5, 8, 3, 8, 2, 9, 4, 5, 8,
            ],
            vec![
                2, 8, 1, 7, 8, 5, 2, 5, 5, 5, 3, 9, 2, 8, 9, 6, 3, 6, 6, 6, 4, 1, 3, 9, 1, 7, 4, 7,
                7, 7, 5, 2, 4, 1, 2, 8, 5, 8, 8, 8, 6, 3, 5, 2, 3, 9, 6, 9, 9, 9,
            ],
            vec![
                5, 7, 5, 4, 5, 6, 3, 5, 7, 2, 6, 8, 6, 5, 6, 7, 4, 6, 8, 3, 7, 9, 7, 6, 7, 8, 5, 7,
                9, 4, 8, 1, 8, 7, 8, 9, 6, 8, 1, 5, 9, 2, 9, 8, 9, 1, 7, 9, 2, 6,
            ],
            vec![
                5, 7, 9, 4, 4, 5, 6, 8, 6, 5, 6, 8, 1, 5, 5, 6, 7, 9, 7, 6, 7, 9, 2, 6, 6, 7, 8, 1,
                8, 7, 8, 1, 3, 7, 7, 8, 9, 2, 9, 8, 9, 2, 4, 8, 8, 9, 1, 3, 1, 9,
            ],
            vec![
                7, 5, 6, 9, 8, 6, 5, 1, 7, 4, 8, 6, 7, 1, 9, 7, 6, 2, 8, 5, 9, 7, 8, 2, 1, 8, 7, 3,
                9, 6, 1, 8, 9, 3, 2, 9, 8, 4, 1, 7, 2, 9, 1, 4, 3, 1, 9, 5, 2, 8,
            ],
            vec![
                5, 6, 4, 7, 5, 7, 3, 9, 6, 5, 6, 7, 5, 8, 6, 8, 4, 1, 7, 6, 7, 8, 6, 9, 7, 9, 5, 2,
                8, 7, 8, 9, 7, 1, 8, 1, 6, 3, 9, 8, 9, 1, 8, 2, 9, 2, 7, 4, 1, 9,
            ],
            vec![
                6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6, 8, 9, 7, 7, 6, 1, 1, 2,
                5, 7, 9, 1, 8, 8, 7, 2, 2, 3, 6, 8, 1, 2, 9, 9, 8, 3, 3, 4, 7, 9,
            ],
        ]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = 40;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_correctly_real_map() {
        let input = sample_input();
        let expected = sample_real_map();
        let actual = compute_real_map(&input, 5);
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual[0].len(), expected[0].len());
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 315;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
