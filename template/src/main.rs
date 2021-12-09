use std::fs::File;
use std::io::prelude::*;

type Input = Vec<u32>;
type Part1Output = usize;
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
    let lines: Input = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| file_fragment.parse::<u32>().unwrap())
        .collect();
    return Ok(lines);
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    return Ok(input.len());
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    return Ok(input.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        vec![42]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "42";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = 1;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 1;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
