use std::fs::File;
use std::io::prelude::*;

type Input = Vec<char>;
type Part1Output = u64;
type Part2Output = u64;

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
        .flat_map(|file_fragment| file_fragment.chars().collect::<Vec<char>>())
        .collect();
    return Ok(lines);
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    let binary_input: Vec<u8> = hex_sequence_to_binary(input);
    let packets = parse_packets(&binary_input);
    let version_sum = sum_version(&packets);
    return Ok(version_sum);
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let binary_input: Vec<u8> = hex_sequence_to_binary(input);
    let packets = parse_packets(&binary_input);
    return Ok(execute_operator(&packets[0]));
}

fn hex_sequence_to_binary(hex: &Vec<char>) -> Vec<u8> {
    hex.iter().flat_map(hex_to_binary).collect()
}

fn hex_to_binary(hex: &char) -> Vec<u8> {
    match hex {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1],
        '8' => vec![1, 0, 0, 0],
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => panic!("cannot convert non hex character"),
    }
}

fn sum_version(packets: &Vec<Packet>) -> u64 {
    packets
        .iter()
        .flat_map(|packet| match packet {
            Packet::LitteralValue(version, _, _) => vec![*version],
            Packet::Operator(version, _, _, _, sub_packets) => {
                vec![*version, sum_version(sub_packets)]
            }
        })
        .sum()
}

type Version = u64;
type Id = u64;
type LitteralValue = u64;
type LengthId = u64;
type Length = u64;

#[derive(Clone, Debug, PartialEq)]
enum Packet {
    LitteralValue(Version, Id, LitteralValue),
    Operator(Version, Id, LengthId, Length, Vec<Packet>),
}

fn parse_packets(transmission: &Vec<u8>) -> Vec<Packet> {
    fn parse_packet(iter: &mut dyn Iterator<Item = &u8>) -> Packet {
        let version_bits: Vec<u8> = take_from_iter(iter, 3); // vec![it.next().unwrap(),it.next().unwrap(),it.next().unwrap()];
        let id_bits: Vec<u8> = take_from_iter(iter, 3); // vec![it.next().unwrap(),it.next().unwrap(),it.next().unwrap()];
        let version: u64 = bits_to_u64(&version_bits);
        let id: u64 = bits_to_u64(&id_bits);

        match id {
            4 => {
                // litteral
                let mut value_bits: Vec<u8> = Vec::new();
                loop {
                    let group_bit: u8 = *iter.next().unwrap();
                    let digit_bits = take_from_iter(iter, 4);
                    digit_bits.iter().for_each(|b| value_bits.push(*b));
                    if group_bit == 0 {
                        break;
                    }
                }
                let value = bits_to_u64(&value_bits);
                return Packet::LitteralValue(version, id, value);
            }
            _ => {
                // operator
                let length_id: u8 = *iter.next().unwrap();
                if length_id == 0 {
                    let length_bits = take_from_iter(iter, 15);
                    let length = bits_to_u64(&length_bits);
                    let sub_packets_bits = take_from_iter(iter, length);
                    let mut sub_iter = sub_packets_bits.iter().peekable();
                    let mut sub_packets: Vec<Packet> = Vec::new();
                    while let Some(_) = sub_iter.peek() {
                        sub_packets.push(parse_packet(&mut sub_iter));
                    }
                    return Packet::Operator(version, id, length_id.into(), length, sub_packets);
                } else {
                    let length_bits = take_from_iter(iter, 11);
                    let length = bits_to_u64(&length_bits);
                    let sub_packets: Vec<Packet> =
                        (0..length).map(|_| parse_packet(iter)).collect();
                    return Packet::Operator(version, id, length_id.into(), length, sub_packets);
                }
            }
        };
    }

    let mut packets: Vec<Packet> = Vec::new();
    let mut it = transmission.iter();
    packets.push(parse_packet(&mut it));
    return packets;
}

fn take_from_iter(iter: &mut dyn Iterator<Item = &u8>, n: u64) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for _ in 0..n {
        res.push(iter.next().unwrap().clone());
    }
    return res;
}

fn bits_to_u64(bits: &Vec<u8>) -> u64 {
    let bits_str = bits.iter().map(|b| format!("{}", b)).collect::<String>();
    let res = u64::from_str_radix(&bits_str, 2);
    if res.is_err() {
        panic!("Cannot convert bits to u64 {:?} {:?}", bits_str, res);
    }

    return res.unwrap();
}

fn execute_operator(packet: &Packet) -> u64 {
    match packet {
        Packet::LitteralValue(_, _, value) => *value,
        Packet::Operator(_, id, _, _, sub_packets) => {
            match id {
                0 =>
                // SUM
                {
                    sub_packets.iter().map(execute_operator).sum()
                }
                1 =>
                // PRODUCT
                {
                    sub_packets.iter().map(execute_operator).product()
                }
                2 =>
                // MIN
                {
                    sub_packets.iter().map(execute_operator).min().unwrap()
                }
                3 =>
                // MAX
                {
                    sub_packets.iter().map(execute_operator).max().unwrap()
                }
                5 => {
                    // GREATER THAN
                    let first: u64 = execute_operator(&sub_packets[0]);
                    let second: u64 = execute_operator(&sub_packets[1]);
                    if first > second {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    // LESS THAN
                    let first: u64 = execute_operator(&sub_packets[0]);
                    let second: u64 = execute_operator(&sub_packets[1]);
                    if first < second {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    // EQUALS
                    let first: u64 = execute_operator(&sub_packets[0]);
                    let second: u64 = execute_operator(&sub_packets[1]);
                    if first == second {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("unknown operator {}", id),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        vec!['D', '2', 'F', 'E', '2', '8']
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "D2FE28\n";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_parse_packet_correctly() {
        let input =
            hex_sequence_to_binary(&parse_data(String::from("8A004A801A8002F478\n")).unwrap());
        println!(
            "{:?}",
            input.iter().map(|b| format!("{}", b)).collect::<String>()
        );
        let expected: Vec<Packet> = vec![Packet::Operator(
            4,
            2,
            1,
            1,
            vec![Packet::Operator(
                1,
                2,
                1,
                1,
                vec![Packet::Operator(
                    5,
                    2,
                    0,
                    11,
                    vec![Packet::LitteralValue(6, 4, 15)],
                )],
            )],
        )];
        assert_eq!(parse_packets(&input), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly_1() {
        let input = parse_data(String::from("8A004A801A8002F478")).unwrap();
        let expected = 16;
        assert_eq!(part1(&input).unwrap(), expected);
    }
    #[test]
    fn it_should_compute_part1_correctly_2() {
        let input = parse_data(String::from("620080001611562C8802118E34")).unwrap();
        let expected = 12;
        assert_eq!(part1(&input).unwrap(), expected);
    }
    #[test]
    fn it_should_compute_part1_correctly_3() {
        let input = parse_data(String::from("C0015000016115A2E0802F182340")).unwrap();
        let expected = 23;
        assert_eq!(part1(&input).unwrap(), expected);
    }
    #[test]
    fn it_should_compute_part1_correctly_4() {
        let input = parse_data(String::from("A0016C880162017C3686B18A3D4780")).unwrap();
        let expected = 31;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly_1() {
        let input = parse_data(String::from("C200B40A82")).unwrap();
        assert_eq!(part2(&input).unwrap(), 3);
    }

    #[test]
    fn it_should_compute_part2_correctly_2() {
        let input = parse_data(String::from("04005AC33890")).unwrap();
        assert_eq!(part2(&input).unwrap(), 54);
    }

    #[test]
    fn it_should_compute_part2_correctly_3() {
        let input = parse_data(String::from("880086C3E88112")).unwrap();
        assert_eq!(part2(&input).unwrap(), 7);
    }

    #[test]
    fn it_should_compute_part2_correctly_4() {
        let input = parse_data(String::from("CE00C43D881120")).unwrap();
        assert_eq!(part2(&input).unwrap(), 9);
    }

    #[test]
    fn it_should_compute_part2_correctly_5() {
        let input = parse_data(String::from("D8005AC2A8F0")).unwrap();
        assert_eq!(part2(&input).unwrap(), 1);
    }

    #[test]
    fn it_should_compute_part2_correctly_6() {
        let input = parse_data(String::from("F600BC2D8F")).unwrap();
        assert_eq!(part2(&input).unwrap(), 0);
    }

    #[test]
    fn it_should_compute_part2_correctly_7() {
        let input = parse_data(String::from("9C005AC2F8F0")).unwrap();
        assert_eq!(part2(&input).unwrap(), 0);
    }

    #[test]
    fn it_should_compute_part2_correctly_8() {
        let input = parse_data(String::from("9C0141080250320F1802104A08")).unwrap();
        assert_eq!(part2(&input).unwrap(), 1);
    }
}
