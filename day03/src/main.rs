use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Bit {
    High,
    Low,
}

fn char_to_bit(c: char) -> Option<Bit> {
    match c {
        '0' => Some(Bit::Low),
        '1' => Some(Bit::High),
        _ => None,
    }
}
fn bit_to_char(bit: &Bit) -> char {
    match bit {
        Bit::High => '1',
        Bit::Low => '0',
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Diagnostic {
    number_width: usize,
    numbers: Vec<String>,
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

fn parse_data(input: String) -> Result<Diagnostic, String> {
    let lines: Vec<String> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(String::from)
        .collect();
    let number_width: usize = lines
        .iter()
        .map(|x| x.len())
        .nth(0)
        .expect("should have at least one element");
    return Ok(Diagnostic {
        number_width: number_width,
        numbers: lines,
    });
}

fn part1(data: &Diagnostic) -> Result<u32, String> {
    let bit_matrix: Vec<Vec<Bit>> = diagnostic_to_bit_matrix(data);
    let column_bit_matrix = convert_to_column_matrix(&bit_matrix, data.number_width);
    let gamma: Vec<Bit> = column_bit_matrix
        .iter()
        .map(|column| {
            let (high, low): (Vec<Bit>, Vec<Bit>) =
                column.iter().partition(|bit| **bit == Bit::High);
            if high.len() >= low.len() {
                return Bit::High;
            } else {
                return Bit::Low;
            }
        })
        .collect();
    let epsilon = permut_bits(&gamma);
    let gamma = bits_to_u32(&gamma);
    let epsilon = bits_to_u32(&epsilon);
    return Ok(gamma * epsilon);
}

fn part2(data: &Diagnostic) -> Result<u32, String> {
    let oxygen = find_number(data, Bit::High, Bit::Low);
    let co2 = find_number(data, Bit::Low, Bit::High);
    return Ok(oxygen * co2);
}

fn diagnostic_to_bit_matrix(data: &Diagnostic) -> Vec<Vec<Bit>> {
    data.numbers
        .iter()
        .map(|x| {
            x.chars()
                .map(char_to_bit)
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect()
        })
        .collect()
}

fn convert_to_column_matrix(matrix: &Vec<Vec<Bit>>, column_count: usize) -> Vec<Vec<Bit>> {
    (Range {
        start: 0,
        end: column_count,
    })
    .map(|i| {
        matrix
            .iter()
            .map(|x| x.iter().nth(i).clone().unwrap())
            .map(|x| (*x).clone())
            .collect()
    })
    .collect()
}

fn permut_bits(bits: &Vec<Bit>) -> Vec<Bit> {
    bits.iter()
        .map(|b| match b {
            Bit::High => Bit::Low,
            Bit::Low => Bit::High,
        })
        .collect()
}

fn bits_to_u32(bits: &Vec<Bit>) -> u32 {
    let binary = bits
        .iter()
        .map(bit_to_char)
        .map(String::from)
        .collect::<Vec<String>>()
        .join("");
    return u32::from_str_radix(binary.as_str(), 2).expect("number should be convertible to u32");
}

fn grab_column(bit_matrix: &Vec<Vec<Bit>>, column_index: usize) -> &Vec<Bit> {
    bit_matrix
        .iter()
        .nth(column_index)
        .expect("column should exist")
}

fn compute_which_bit_to_keep(
    column: &Vec<Bit>,
    high_most_present: Bit,
    low_most_present: Bit,
) -> Bit {
    let (high, low): (Vec<Bit>, Vec<Bit>) = column.iter().partition(|bit| **bit == Bit::High);
    if high.len() >= low.len() {
        return high_most_present;
    } else {
        return low_most_present;
    }
}

fn filter_matrix_with_keep_bit(
    bit_matrix: &Vec<Vec<Bit>>,
    column_index: usize,
    keep: Bit,
) -> Vec<Vec<Bit>> {
    bit_matrix
        .iter()
        .filter(|n| *(n.iter().nth(column_index).unwrap()) == keep)
        .map(|n| n.clone())
        .collect()
}

fn find_number(data: &Diagnostic, high_most_present: Bit, low_most_present: Bit) -> u32 {
    let mut bit_matrix: Vec<Vec<Bit>> = diagnostic_to_bit_matrix(data);
    let mut index = 0;
    while bit_matrix.len() > 1 && index < data.number_width {
        let column_bit_matrix = convert_to_column_matrix(&bit_matrix, data.number_width);
        let column = grab_column(&column_bit_matrix, index);
        let keep = compute_which_bit_to_keep(&column, high_most_present, low_most_present);
        bit_matrix = filter_matrix_with_keep_bit(&bit_matrix, index, keep);
        index += 1;
    }
    return bits_to_u32(&(bit_matrix[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly() {
        let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let expected = Diagnostic {
            number_width: 5,
            numbers: vec![
                String::from("00100"),
                String::from("11110"),
                String::from("10110"),
                String::from("10111"),
                String::from("10101"),
                String::from("01111"),
                String::from("00111"),
                String::from("11100"),
                String::from("10000"),
                String::from("11001"),
                String::from("00010"),
                String::from("01010"),
            ],
        };
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let data = Diagnostic {
            number_width: 5,
            numbers: vec![
                String::from("00100"),
                String::from("11110"),
                String::from("10110"),
                String::from("10111"),
                String::from("10101"),
                String::from("01111"),
                String::from("00111"),
                String::from("11100"),
                String::from("10000"),
                String::from("11001"),
                String::from("00010"),
                String::from("01010"),
            ],
        };
        let expected = 198;
        assert_eq!(part1(&data).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let data = Diagnostic {
            number_width: 5,
            numbers: vec![
                String::from("00100"),
                String::from("11110"),
                String::from("10110"),
                String::from("10111"),
                String::from("10101"),
                String::from("01111"),
                String::from("00111"),
                String::from("11100"),
                String::from("10000"),
                String::from("11001"),
                String::from("00010"),
                String::from("01010"),
            ],
        };
        let expected = 230;
        assert_eq!(part2(&data).unwrap(), expected);
    }
}
