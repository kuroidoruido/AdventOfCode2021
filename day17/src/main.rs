use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Target {
    from_x: i64,
    end_x: i64,
    from_y: i64,
    end_y: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PositionStatus {
    Before,
    OnTarget,
    After,
}

type Input = Target;
type Part1Output = i64;
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
    let re = Regex::new(r"target area: x=([-0-9]+)..([-0-9]+), y=([-0-9]+)..([-0-9]+)").unwrap();
    let lines: Vec<&str> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .collect();
    let captured = re.captures(lines[0]).unwrap();

    let from_x: i64 = i64::from_str_radix(captured.get(1).unwrap().as_str(), 10).unwrap();
    let end_x: i64 = i64::from_str_radix(captured.get(2).unwrap().as_str(), 10).unwrap();
    let from_y: i64 = i64::from_str_radix(captured.get(3).unwrap().as_str(), 10).unwrap();
    let end_y: i64 = i64::from_str_radix(captured.get(4).unwrap().as_str(), 10).unwrap();
    return Ok(Target {
        from_x,
        end_x,
        from_y,
        end_y,
    });
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    let reached_target = get_parameter_to_reach_target(input);

    return Ok(reached_target
        .iter()
        .map(|(_, max_y)| *max_y)
        .max()
        .unwrap());
}

fn get_position_status(target: Target, position: (i64, i64)) -> PositionStatus {
    if position.0 < target.from_x {
        if position.1 < target.from_y {
            return PositionStatus::After;
        } else {
            return PositionStatus::Before;
        }
    } else if target.end_x < position.0 {
        return PositionStatus::After;
    } else {
        if position.1 < target.from_y {
            return PositionStatus::After;
        } else if target.end_y < position.1 {
            return PositionStatus::Before;
        } else {
            return PositionStatus::OnTarget;
        }
    }
}

fn get_parameter_to_reach_target(input: &Target) -> Vec<((i64, i64), i64)> {
    let start_position: (i64, i64) = (0, 0);
    let mut reached_target: Vec<((i64, i64), i64)> = Vec::new();
    // these two for loops are using pretty much magic random numbers...
    for start_x in (-input.end_x * 2)..(input.end_x * 2) {
        for start_y in (-input.from_y.abs() * 2)..(input.from_y.abs() * 2) {
            let mut x = start_x;
            let mut y = start_y;
            let mut max_y = 0;
            let mut current_position = start_position.clone();
            loop {
                if max_y < current_position.1 {
                    max_y = current_position.1;
                }
                let status = get_position_status(*input, current_position);
                match status {
                    PositionStatus::Before => {
                        current_position = (current_position.0 + x, current_position.1 + y);
                        if x < 0 {
                            x = x + 1;
                        } else if x > 0 {
                            x = x - 1;
                        }
                        y = y - 1;
                    }
                    PositionStatus::OnTarget => {
                        reached_target.push((current_position, max_y));
                        break;
                    }
                    PositionStatus::After => break,
                }
            }
        }
    }
    return reached_target;
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let reached_target = get_parameter_to_reach_target(input);
    return Ok(reached_target.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        Target {
            from_x: 20,
            end_x: 30,
            from_y: -10,
            end_y: -5,
        }
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "target area: x=20..30, y=-10..-5\n";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_detect_as_before_position_with_x_in_target_but_y_over() {
        let target = sample_input();
        let position: (i64, i64) = (25, 2);
        assert_eq!(
            get_position_status(target, position),
            PositionStatus::Before
        );
    }

    #[test]
    fn it_should_detect_as_before_position_with_y_in_target_but_x_before() {
        let target = sample_input();
        let position: (i64, i64) = (18, -8);
        assert_eq!(
            get_position_status(target, position),
            PositionStatus::Before
        );
    }

    #[test]
    fn it_should_detect_as_ontarget_position_with_x_and_y_on_target() {
        let target = sample_input();
        let position: (i64, i64) = (25, -8);
        assert_eq!(
            get_position_status(target, position),
            PositionStatus::OnTarget
        );
    }

    #[test]
    fn it_should_detect_as_after_position_with_x_after_target() {
        let target = sample_input();
        let position: (i64, i64) = (31, -8);
        assert_eq!(get_position_status(target, position), PositionStatus::After);
    }

    #[test]
    fn it_should_detect_as_after_position_with_y_bellow_target() {
        let target = sample_input();
        let position: (i64, i64) = (25, -11);
        assert_eq!(get_position_status(target, position), PositionStatus::After);
    }

    #[test]
    fn it_should_detect_as_after_position_with_y_bellow_target_and_x_before() {
        let target = sample_input();
        let position: (i64, i64) = (10, -11);
        assert_eq!(get_position_status(target, position), PositionStatus::After);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = 45;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 112;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
