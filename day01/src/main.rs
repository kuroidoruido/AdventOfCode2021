use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

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

fn parse_data(input: String) -> Result<Vec<u32>, String> {
    let lines: Vec<u32> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| file_fragment.parse::<u32>().unwrap())
        .collect();
    return Ok(lines);
}

fn part1(data: &Vec<u32>) -> Result<usize, String> {
    return Ok(data
        .iter()
        .tuple_windows::<(&u32, &u32)>()
        .filter(|(x, y)| x < y)
        .count());
}

fn part2(data: &Vec<u32>) -> Result<usize, String> {
    return part1(
        &data
            .iter()
            .tuple_windows::<(&u32, &u32, &u32)>()
            .map(|(x, y, z)| x + y + z)
            .collect::<Vec<u32>>(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        let expected = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;
        assert_eq!(part1(&data).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 5;
        assert_eq!(part2(&data).unwrap(), expected);
    }
}
