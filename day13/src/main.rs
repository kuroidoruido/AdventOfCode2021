use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Mark {
    Dot,
    No,
}

impl Mark {
    fn is_dot(self) -> bool {
        match self {
            Mark::Dot => true,
            Mark::No => false,
        }
    }
}

impl core::fmt::Debug for Mark {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Mark::Dot => write!(f, "█"),
            Mark::No => write!(f, "░"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Instruction {
    X(usize),
    Y(usize),
}

#[derive(Clone, PartialEq, Debug)]
struct Input {
    page: Vec<Vec<Mark>>,
    instructions: Vec<Instruction>,
}

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
    let lines: Vec<&str> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .collect();

    let mut it = lines.iter();
    let mut dot_position: Vec<(usize, usize)> = Vec::new();
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(",").expect("should have a , separator");
        dot_position.push((
            x.parse::<usize>().expect("x should be a number"),
            y.parse::<usize>().expect("y should be a number"),
        ));
    }
    let max_x: usize = *dot_position.iter().map(|(x, _)| x).max().unwrap();
    let max_y: usize = *dot_position.iter().map(|(_, y)| y).max().unwrap();
    let mut page: Vec<Vec<Mark>> = vec![vec![Mark::No; max_x + 1]; max_y + 1];
    dot_position
        .iter()
        .for_each(|(x, y)| page[*y][*x] = Mark::Dot);

    let mut instructions: Vec<Instruction> = Vec::new();
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        if line.starts_with("fold along x=") {
            let (_, fold_position) = line.split_once("=").unwrap();
            instructions.push(Instruction::X(
                fold_position
                    .parse::<usize>()
                    .expect("X should be a number"),
            ));
        } else if line.starts_with("fold along y=") {
            let (_, fold_position) = line.split_once("=").unwrap();
            instructions.push(Instruction::Y(
                fold_position
                    .parse::<usize>()
                    .expect("Y should be a number"),
            ));
        }
    }

    return Ok(Input { page, instructions });
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    let new_matrix = fold(&input.page, input.instructions[0]);
    return Ok(count_dot(&new_matrix));
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let mut new_matrix: Vec<Vec<Mark>> = input.page.clone();
    input
        .instructions
        .iter()
        .for_each(|inst| new_matrix = fold(&new_matrix, *inst));
    print_matrix(&new_matrix);
    return Ok(0);
}

fn fold(matrix: &Vec<Vec<Mark>>, instruction: Instruction) -> Vec<Vec<Mark>> {
    return match instruction {
        Instruction::X(x_fold) => {
            let (matrix_left, matrix_right) = cut_matrix_x(matrix, x_fold);
            let matrix_right = x_mirror(&matrix_right);
            merge_dot_matrixes_from_bottom_right(&matrix_left, &matrix_right)
        }
        Instruction::Y(y_fold) => {
            let (matrix_top, matrix_bottom) = cut_matrix_y(matrix, y_fold);
            let matrix_bottom = y_mirror(&matrix_bottom);
            merge_dot_matrixes_from_bottom_right(&matrix_top, &matrix_bottom)
        }
    };
}

fn cut_matrix_x<T: Copy>(matrix: &Vec<Vec<T>>, x: usize) -> (Vec<Vec<T>>, Vec<Vec<T>>) {
    (
        matrix
            .iter()
            .map(|row| row.iter().take(x).copied().collect())
            .collect(),
        matrix
            .iter()
            .map(|row| row.iter().skip(x + 1).copied().collect())
            .collect(),
    )
}

fn cut_matrix_y<T: Clone>(matrix: &Vec<Vec<T>>, y: usize) -> (Vec<Vec<T>>, Vec<Vec<T>>) {
    (
        matrix.iter().take(y).cloned().collect(),
        matrix.iter().skip(y + 1).cloned().collect(),
    )
}

fn x_mirror<T: Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    matrix
        .iter()
        .map(|row| row.iter().rev().copied().collect::<Vec<T>>())
        .collect()
}

fn y_mirror<T: Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    matrix.iter().rev().cloned().collect()
}

fn print_matrix<T: core::fmt::Debug>(matrix: &Vec<Vec<T>>) {
    println!("----");
    matrix.iter().for_each(|row| {
        row.iter().for_each(|dot| print!("{:?}", dot));
        println!("");
    });
    println!("----");
}

fn merge_dot_matrixes_from_bottom_right(
    matrix1: &Vec<Vec<Mark>>,
    matrix2: &Vec<Vec<Mark>>,
) -> Vec<Vec<Mark>> {
    let mut matrix: Vec<Vec<Mark>> = vec![vec![Mark::No; matrix1[0].len()]; matrix1.len()];
    let m1_max_x = matrix1[0].len() - 1;
    let m1_max_y = matrix1.len() - 1;
    let m2_max_x = matrix2[0].len() - 1;
    let m2_max_y = matrix2.len() - 1;
    let folded_x = vec![m1_max_x, m2_max_x];
    let folded_y = vec![m1_max_y, m2_max_y];
    let folded_x = folded_x.iter().min().unwrap();
    let folded_y = folded_y.iter().min().unwrap();
    for j in 0..(folded_y + 1) {
        for i in 0..(folded_x + 1) {
            if matrix1[m1_max_y - j][m1_max_x - i].is_dot()
                || matrix2[m2_max_y - j][m2_max_x - i].is_dot()
            {
                matrix[m1_max_y - j][m1_max_x - i] = Mark::Dot;
            } else {
                matrix[m1_max_y - j][m1_max_x - i] = Mark::No;
            }
        }
    }
    return matrix;
}

fn count_dot(matrix: &Vec<Vec<Mark>>) -> usize {
    matrix
        .iter()
        .flat_map(|row| row)
        .filter(|mark| mark.is_dot())
        .count()
}

#[cfg(test)]
mod tests {
    use super::Instruction::*;
    use super::Mark::*;
    use super::*;

    fn sample_input() -> Input {
        Input {
            page: vec![
                vec![No, No, No, Dot, No, No, Dot, No, No, Dot, No],
                vec![No, No, No, No, Dot, No, No, No, No, No, No],
                vec![No, No, No, No, No, No, No, No, No, No, No],
                vec![Dot, No, No, No, No, No, No, No, No, No, No],
                vec![No, No, No, Dot, No, No, No, No, Dot, No, Dot],
                vec![No, No, No, No, No, No, No, No, No, No, No],
                vec![No, No, No, No, No, No, No, No, No, No, No],
                vec![No, No, No, No, No, No, No, No, No, No, No],
                vec![No, No, No, No, No, No, No, No, No, No, No],
                vec![No, No, No, No, No, No, No, No, No, No, No],
                vec![No, Dot, No, No, No, No, Dot, No, Dot, Dot, No],
                vec![No, No, No, No, Dot, No, No, No, No, No, No],
                vec![No, No, No, No, No, No, Dot, No, No, No, Dot],
                vec![Dot, No, No, No, No, No, No, No, No, No, No],
                vec![Dot, No, Dot, No, No, No, No, No, No, No, No],
            ],
            instructions: vec![Y(7), X(5)],
        }
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = 17;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 1;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
