use std::fs::File;
use std::io::prelude::*;

type CrabPosition = u32;

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

fn parse_data(input: String) -> Result<Vec<CrabPosition>, String> {
    let (fishes, _) = input
        .split_once("\n")
        .expect("should have at least one line");
    return Ok(fishes
        .split(',')
        .map(str::trim)
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| file_fragment.parse::<CrabPosition>().unwrap())
        .collect());
}

fn part1(data: &Vec<CrabPosition>) -> Result<u32, String> {
    return Ok(compute_best_fuel_cost(data, |move_distance| move_distance));
}

fn part2(data: &Vec<CrabPosition>) -> Result<u32, String> {
    return Ok(compute_best_fuel_cost(data, |move_distance| {
        move_distance * (move_distance + 1) / 2
    }));
}

fn compute_best_fuel_cost(data: &Vec<CrabPosition>, compute_move_cost: fn(u32) -> u32) -> u32 {
    let min = *data.iter().min().expect("should have at least one element");
    let max = *data.iter().max().expect("should have at least one element");
    (min..max)
        .map(|destination| {
            data.iter()
                .map(|crab_position| {
                    let step = if destination > *crab_position {
                        destination - crab_position
                    } else {
                        crab_position - destination
                    };
                    compute_move_cost(step)
                })
                .sum()
        })
        .min()
        .expect("should have at least one element")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<CrabPosition> {
        vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "16,1,2,0,4,2,7,1,2,14\n";
        let expected = sample_data();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let data = sample_data();
        let expected = 37;
        assert_eq!(part1(&data).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let data = sample_data();
        let expected = 168;
        assert_eq!(part2(&data).unwrap(), expected);
    }
}
