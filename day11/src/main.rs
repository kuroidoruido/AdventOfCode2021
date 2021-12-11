use std::fs::File;
use std::io::prelude::*;

type Input = Vec<Vec<u8>>;
type Part1Output = usize;
type Part2Output = u32;

fn main() -> std::io::Result<()> {
    let input1 = read_input("input1.txt").expect("An error occurred when reading input1.txt");
    let data1 = parse_data(input1).expect("An error occurred when parsing input1.txt");

    println!("Part 1: {:?}", part1(&data1, 100));
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
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();
    return Ok(lines);
}

fn part1(state: &Input, iteration_count: u32) -> Result<Part1Output, String> {
    if iteration_count == 0 {
        return Ok(0);
    }
    let new_state: Input = increase_all_energy(&state);
    let flash_count = get_flash_count(&new_state);

    if let Ok(next_step_flashes_count) = part1(&new_state, iteration_count - 1) {
        return Ok(flash_count + next_step_flashes_count);
    } else {
        return Err(format!(
            "Did not success to compute step: {}",
            iteration_count - 1
        ));
    }
}

fn increase_all_energy(state: &Input) -> Input {
    fn increase_adjecent_with_flash(local_state: &mut Input, i: usize, j: usize) {
        if local_state[j][i] == 11 {
            // we already apply flash on adjacent
            return;
        }
        let adjacents = get_adjacents(&local_state, i, j);
        adjacents.iter().map(|(_, pos)| pos).for_each(|(x, y)| {
            // when over 9, octopus flashing yet, so non necessary to increase energy
            if local_state[*y][*x] < 10 {
                local_state[*y][*x] = local_state[*y][*x] + 1;
                if local_state[*y][*x] > 9 {
                    increase_adjecent_with_flash(local_state, *x, *y);
                    local_state[*y][*x] = 11;
                }
            }
        });
    }

    let mut new_state: Input = Vec::with_capacity(state.len());
    // increase all octopus enery by 1
    for actual_row in state.iter() {
        new_state.push(actual_row.iter().map(|x| x + 1).collect());
    }

    // increase energy again for adjacent of flashing octopus
    for j in 0..new_state.len() {
        for i in 0..new_state[0].len() {
            if new_state[j][i] > 9 {
                increase_adjecent_with_flash(&mut new_state, i, j);
                new_state[j][i] = 11;
            }
        }
    }
    return new_state
        .iter()
        .map(|row| {
            row.iter()
                .map(|x| if *x > 9 { 0 } else { *x })
                .collect::<Vec<u8>>()
        })
        .collect();
}

fn get_flash_count(state: &Input) -> usize {
    state.iter().flat_map(|x| x).filter(|x| **x == 0).count()
}

fn get_adjacents(data: &Input, x: usize, y: usize) -> Vec<(u8, (usize, usize))> {
    let max_y: isize = (data.len() - 1)
        .try_into()
        .expect("should be able to convert max_y to isize");
    let max_x: isize = (data[0].len() - 1)
        .try_into()
        .expect("should be able to convert max_x to isize");
    let x: isize = x.try_into().expect("should be able to convert x to isize");
    let y: isize = y.try_into().expect("should be able to convert y to isize");
    let adjacent_positions: Vec<(isize, isize)> = vec![
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ];
    adjacent_positions
        .iter()
        .filter(|(ax, ay)| *ax >= 0 && *ay >= 0 && *ax <= max_x && *ay <= max_y)
        .map(|(ax, ay)| {
            let ax: usize = usize::try_from(*ax).unwrap();
            let ay: usize = usize::try_from(*ay).unwrap();
            (ax, ay)
        })
        .map(|(ax, ay)| (data[ay][ax], (ax, ay)))
        .collect()
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let expected_flash_count = input.iter().map(|row| row.len()).sum();

    let mut step: u32 = 1;
    let mut new_state: Input = increase_all_energy(input);
    while get_flash_count(&new_state) != expected_flash_count {
        step += 1;
        new_state = increase_all_energy(&new_state);
    }
    return Ok(step);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
        ";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_correctly_flash_count() {
        let input = vec![
            vec![3, 4, 5, 4, 3],
            vec![4, 0, 0, 0, 4],
            vec![5, 0, 0, 0, 5],
            vec![4, 0, 0, 0, 4],
            vec![3, 4, 5, 4, 3],
        ];
        let expected = 9;
        assert_eq!(get_flash_count(&input), expected);
    }

    #[test]
    fn it_should_increase_correctly_energy_1() {
        println!("it_should_increase_correctly_energy_1");
        let input = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];
        let expected = vec![
            vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
            vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
            vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
            vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
            vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
            vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
            vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
            vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
            vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
            vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
        ];
        assert_eq!(increase_all_energy(&input), expected);
    }

    #[test]
    fn it_should_increase_correctly_energy_2() {
        println!("it_should_increase_correctly_energy_2");
        let input = vec![
            vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
            vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
            vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
            vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
            vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
            vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
            vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
            vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
            vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
            vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
        ];
        let expected = vec![
            vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
            vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
            vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
            vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
            vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
            vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
            vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
            vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
            vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
            vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
        ];
        let actual = increase_all_energy(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_should_increase_correctly_energy_3() {
        println!("it_should_increase_correctly_energy_3");
        let input = vec![
            vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
            vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
            vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
            vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
            vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
            vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
            vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
            vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
            vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
            vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
        ];
        let expected = vec![
            vec![0, 0, 5, 0, 9, 0, 0, 8, 6, 6],
            vec![8, 5, 0, 0, 8, 0, 0, 5, 7, 5],
            vec![9, 9, 0, 0, 0, 0, 0, 0, 3, 9],
            vec![9, 7, 0, 0, 0, 0, 0, 0, 4, 1],
            vec![9, 9, 3, 5, 0, 8, 0, 0, 6, 3],
            vec![7, 7, 1, 2, 3, 0, 0, 0, 0, 0],
            vec![7, 9, 1, 1, 2, 5, 0, 0, 0, 9],
            vec![2, 2, 1, 1, 1, 3, 0, 0, 0, 0],
            vec![0, 4, 2, 1, 1, 2, 5, 0, 0, 0],
            vec![0, 0, 2, 1, 1, 1, 9, 0, 0, 0],
        ];
        assert_eq!(increase_all_energy(&input), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly_10() {
        println!("it_should_compute_part1_correctly_10");
        let input = sample_input();
        let expected = 204;
        assert_eq!(part1(&input, 10).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly_100() {
        let input = sample_input();
        let expected = 1656;
        assert_eq!(part1(&input, 100).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 195;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
