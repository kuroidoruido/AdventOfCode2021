use std::cmp;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Line {
    from: (u32, u32),
    to: (u32, u32),
}

impl Line {
    fn parse(s: &str) -> Result<Line, String> {
        if let Some((from_str, to_str)) = s.split_once(" -> ") {
            if let Some((x1_str, y1_str)) = from_str.split_once(',') {
                if let Some((x2_str, y2_str)) = to_str.split_once(',') {
                    let x1 = x1_str.parse::<u32>();
                    let y1 = y1_str.parse::<u32>();
                    let x2 = x2_str.parse::<u32>();
                    let y2 = y2_str.parse::<u32>();
                    if x1.is_ok() && y1.is_ok() && x2.is_ok() && y2.is_ok() {
                        return Ok(Line {
                            from: (x1.unwrap(), y1.unwrap()),
                            to: (x2.unwrap(), y2.unwrap()),
                        });
                    }
                }
            }
        }
        return Err(String::from(
            "the input does not follow expected format: x1,y1 -> x2,y2",
        ));
    }
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

fn parse_data(input: String) -> Result<Vec<Line>, String> {
    let lines: Vec<Line> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(Line::parse)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();
    return Ok(lines);
}

fn part1(data: &Vec<Line>) -> Result<usize, String> {
    let (matrix_width, matrix_height) = get_matrix_dimension(data);
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; matrix_width]; matrix_height];
    let mut data_it = data.iter();
    while let Some(line) = data_it.next() {
        if line.from.0 == line.to.0 {
            let y: usize = line.from.0.try_into().unwrap();
            let min_x: usize = line.from.1.try_into().unwrap();
            let max_x: usize = line.to.1.try_into().unwrap();
            let (min_x, max_x) = ensure_order(min_x, max_x);
            for x in min_x..(max_x + 1) {
                matrix[x][y] = matrix[x][y] + 1;
            }
        } else if line.from.1 == line.to.1 {
            let x: usize = line.from.1.try_into().unwrap();
            let min_y: usize = line.from.0.try_into().unwrap();
            let max_y: usize = line.to.0.try_into().unwrap();
            let (min_y, max_y) = ensure_order(min_y, max_y);
            for y in min_y..(max_y + 1) {
                matrix[x][y] = matrix[x][y] + 1;
            }
        }
    }
    // print_matrix(&matrix);
    return Ok(count_point_over(&matrix, 1));
}

fn part2(data: &Vec<Line>) -> Result<usize, String> {
    let (matrix_height, matrix_width) = get_matrix_dimension(data);
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; matrix_width]; matrix_height];
    let mut data_it = data.iter();
    while let Some(line) = data_it.next() {
        if line.from.0 == line.to.0 {
            let y: usize = line.from.0.try_into().unwrap();
            let min_x: usize = line.from.1.try_into().unwrap();
            let max_x: usize = line.to.1.try_into().unwrap();
            let (min_x, max_x) = ensure_order(min_x, max_x);
            for x in min_x..(max_x + 1) {
                matrix[x][y] = matrix[x][y] + 1;
            }
        } else if line.from.1 == line.to.1 {
            let x: usize = line.from.1.try_into().unwrap();
            let min_y: usize = line.from.0.try_into().unwrap();
            let max_y: usize = line.to.0.try_into().unwrap();
            let (min_y, max_y) = ensure_order(min_y, max_y);
            for y in min_y..(max_y + 1) {
                matrix[x][y] = matrix[x][y] + 1;
            }
        } else {
            let min_x: i32 = line.from.1.try_into().unwrap();
            let min_y: i32 = line.from.0.try_into().unwrap();
            let max_x: i32 = line.to.1.try_into().unwrap();
            let max_y: i32 = line.to.0.try_into().unwrap();
            let direction_x = max_x - min_x;
            let direction_y = max_y - min_y;
            if (direction_x).abs() == (direction_y).abs() {
                let max_increment = (direction_x).abs() + 1;
                let x_sign: i32 = if min_x <= max_x { 1 } else { -1 };
                let y_sign: i32 = if min_y <= max_y { 1 } else { -1 };
                for i in 0..max_increment {
                    let x = min_x + (i * x_sign);
                    let y = min_y + (i * y_sign);
                    if x >= 0 && y >= 0 {
                        let x: usize = x.try_into().unwrap();
                        let y: usize = y.try_into().unwrap();
                        matrix[x][y] = matrix[x][y] + 1;
                    }
                }
            }
        }
    }
    // print_matrix(&matrix);
    return Ok(count_point_over(&matrix, 1));
}

fn get_matrix_dimension(data: &Vec<Line>) -> (usize, usize) {
    data.iter()
        .flat_map(|line| vec![line.from, line.to])
        .map(|(x, y)| (x + 1, y + 1))
        .fold((0, 0), |(max_x, max_y), (cur_x, cur_y)| {
            (
                cmp::max(max_x, cur_x.try_into().unwrap()),
                cmp::max(max_y, cur_y.try_into().unwrap()),
            )
        })
}

fn count_point_over(matrix: &Vec<Vec<u32>>, over: u32) -> usize {
    matrix
        .iter()
        .flat_map(|row| row)
        .filter(|point| **point > over)
        .count()
}

fn ensure_order<T: Ord>(min: T, max: T) -> (T, T) {
    if min <= max {
        (min, max)
    } else {
        (max, min)
    }
}

fn print_matrix(matrix: &Vec<Vec<u32>>) {
    matrix.iter().for_each(|row| {
        row.iter().for_each(|p| {
            if *p == 0 {
                print!(".");
            } else {
                print!("{}", p);
            }
        });
        println!("");
    });
}
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<Line> {
        vec![
            Line {
                from: (0, 9),
                to: (5, 9),
            },
            Line {
                from: (8, 0),
                to: (0, 8),
            },
            Line {
                from: (9, 4),
                to: (3, 4),
            },
            Line {
                from: (2, 2),
                to: (2, 1),
            },
            Line {
                from: (7, 0),
                to: (7, 4),
            },
            Line {
                from: (6, 4),
                to: (2, 0),
            },
            Line {
                from: (0, 9),
                to: (2, 9),
            },
            Line {
                from: (3, 4),
                to: (1, 4),
            },
            Line {
                from: (0, 0),
                to: (8, 8),
            },
            Line {
                from: (5, 5),
                to: (8, 2),
            },
        ]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

        let expected = sample_data();

        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_matrix_dimension_correctly() {
        let data = sample_data();
        assert_eq!(get_matrix_dimension(&data), (10, 10))
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let data = sample_data();
        let expected = 5;
        assert_eq!(part1(&data).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let data = sample_data();
        let expected = 12;
        assert_eq!(part2(&data).unwrap(), expected);
    }
}
