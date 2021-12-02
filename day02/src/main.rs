use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    horizontal_position: u32,
    depth: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position2 {
    horizontal_position: u32,
    depth: u32,
    aim: u32,
}

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

fn parse_data(input: String) -> Result<Vec<Command>, String> {
    let lines: Vec<Command> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| file_fragment.split_once(" "))
        .filter(Option::is_some)
        .map(|splitted| {
            if let Some((command, x)) = splitted {
                let n: u32 = x
                    .parse()
                    .expect(format!("Should be a number: {:?}", x).as_str());
                match command {
                    "forward" => Some(Command::Forward(n)),
                    "down" => Some(Command::Down(n)),
                    "up" => Some(Command::Up(n)),
                    _ => None,
                }
            } else {
                None
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();
    return Ok(lines);
}

fn part1(data: &Vec<Command>) -> Result<u32, String> {
    let position = data.iter().fold(
        Position {
            horizontal_position: 0,
            depth: 0,
        },
        |position, command| match command {
            Command::Forward(x) => Position {
                horizontal_position: position.horizontal_position + x,
                depth: position.depth,
            },
            Command::Down(x) => Position {
                horizontal_position: position.horizontal_position,
                depth: position.depth + x,
            },
            Command::Up(x) => Position {
                horizontal_position: position.horizontal_position,
                depth: position.depth - x,
            },
        },
    );
    return Ok(position.horizontal_position * position.depth);
}

fn part2(data: &Vec<Command>) -> Result<u32, String> {
    let position = data.iter().fold(
        Position2 {
            horizontal_position: 0,
            depth: 0,
            aim: 0,
        },
        |position, command| match command {
            Command::Forward(x) => Position2 {
                horizontal_position: position.horizontal_position + x,
                depth: position.depth + (x * position.aim),
                aim: position.aim,
            },
            Command::Down(x) => Position2 {
                horizontal_position: position.horizontal_position,
                depth: position.depth,
                aim: position.aim + x,
            },
            Command::Up(x) => Position2 {
                horizontal_position: position.horizontal_position,
                depth: position.depth,
                aim: position.aim - x,
            },
        },
    );
    return Ok(position.horizontal_position * position.depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly() {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        let expected: Vec<Command> = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let data = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let expected = 150;
        assert_eq!(part1(&data).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let data = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let expected = 900;
        assert_eq!(part2(&data).unwrap(), expected);
    }
}
