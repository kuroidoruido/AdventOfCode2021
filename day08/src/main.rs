#![feature(option_result_contains)]
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct NoteLine {
    signal_pattern: Vec<String>,
    digits: Vec<String>,
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

fn parse_data(input: String) -> Result<Vec<NoteLine>, String> {
    let lines: Vec<NoteLine> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| {
            file_fragment
                .split_once(" | ")
                .expect("should have | separator")
        })
        .map(|(signal_pattern, digits)| {
            (
                signal_pattern
                    .split(" ")
                    .map(String::from)
                    .collect::<Vec<String>>(),
                digits.split(" ").map(String::from).collect::<Vec<String>>(),
            )
        })
        .map(|(signal_pattern, digits)| NoteLine {
            signal_pattern,
            digits,
        })
        .collect();
    return Ok(lines);
}

fn part1(data: &Vec<NoteLine>) -> Result<usize, String> {
    return Ok(data
        .iter()
        .flat_map(|note_line| note_line.digits.iter())
        .filter(
            |digit| {
                digit.len() == 2 // 1
            ||digit.len() == 4 // 4
            ||digit.len() == 3 // 7
            || digit.len() == 7
            }, // 8
        )
        .count());
}

fn part2(data: &Vec<NoteLine>) -> Result<u32, String> {
    Ok(data.iter().map(find_number_from_pattern).sum())
}

fn build_possible_wire_map() -> HashMap<char, Vec<char>> {
    let mut possible_wire: HashMap<char, Vec<char>> = HashMap::new();
    let wires = all_wired();
    wires.iter().for_each(|wire| {
        possible_wire.insert(*wire, wires.clone());
    });
    return possible_wire;
}

fn known_possible_wire(
    possible_wire: &mut HashMap<char, Vec<char>>,
    wires: Vec<char>,
    one_of: Vec<char>,
) {
    for (wire, maybe_wire) in possible_wire.iter_mut() {
        if wires.contains(wire) {
            maybe_wire.retain(|x| one_of.contains(x));
        } else {
            maybe_wire.retain(|x| !one_of.contains(x));
        }
    }
}

/*
 aaaa
b    c
b    c
 dddd
e    f
e    f
 gggg

*/

fn all_wired() -> Vec<char> {
    vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']
}

fn find_wire_mapping(line: &NoteLine) -> HashMap<char, char> {
    let mut possible_wire = build_possible_wire_map();
    let one = line
        .signal_pattern
        .iter()
        .find(|pattern| pattern.len() == 2)
        .expect("should have the one pattern");
    let seven = line
        .signal_pattern
        .iter()
        .find(|pattern| pattern.len() == 3)
        .expect("should have the seven pattern");
    let four = line
        .signal_pattern
        .iter()
        .find(|pattern| pattern.len() == 4)
        .expect("should have the four pattern");
    let eight = line
        .signal_pattern
        .iter()
        .find(|pattern| pattern.len() == 7)
        .expect("should have the eight pattern");
    // other patterns are for 0,2,3,5,6,9
    let other_patterns: Vec<String> = line
        .signal_pattern
        .iter()
        .filter(|pattern| {
            *pattern != one && *pattern != seven && *pattern != four && *pattern != eight
        })
        .map(String::clone)
        .collect();
    let one_chars: Vec<char> = one.chars().collect();

    // we keep for c and f wire only wire of one
    known_possible_wire(&mut possible_wire, vec!['c', 'f'], one_chars.clone());
    // the only different wire between one and seven is the a wire
    known_possible_wire(
        &mut possible_wire,
        vec!['a'],
        seven.chars().filter(|x| !one_chars.contains(x)).collect(),
    );
    // the two different wire between one and four is b and d
    let b_or_d: Vec<char> = four.chars().filter(|x| !one_chars.contains(x)).collect();
    known_possible_wire(&mut possible_wire, vec!['b', 'd'], b_or_d.clone());
    // from other pattern we can find 0,2,3 which does not show both b and d wires
    let zero_two_or_three: Vec<String> = other_patterns
        .iter()
        .filter(|pattern| !b_or_d.iter().all(|wire| pattern.contains(*wire)))
        .map(String::clone)
        .collect();
    // from 0,2,3 we can find easily 0 because it has 6 wire where 2 and 3 has 5 wire
    let (zero, two_or_three): (Vec<String>, Vec<String>) = zero_two_or_three
        .iter()
        .cloned()
        .partition(|pattern| pattern.len() == 6);
    let zero = zero.get(0).expect("should have found 0");
    // only one wire of 4 is not shown for zero, it's the d wire
    // so we can deduce d (and b)
    let d = four
        .chars()
        .find(|wire| !zero.contains(*wire))
        .expect("should have found d");
    known_possible_wire(&mut possible_wire, vec!['d'], vec![d]);
    // from 2 and 3, only 2 show e and g, only 3 show c and f
    let possible_wire_clone = possible_wire.clone();
    let e_possibilities = possible_wire_clone
        .get(&'e')
        .expect("should have some possibility for e");
    let two = two_or_three
        .iter()
        .find(|pattern| e_possibilities.iter().all(|wire| pattern.contains(*wire)))
        .expect("should find 2");
    let f_possibilities = possible_wire_clone
        .get(&'f')
        .expect("should have some possibility for f");
    let three = two_or_three
        .iter()
        .find(|pattern| f_possibilities.iter().all(|wire| pattern.contains(*wire)))
        .expect("should find 3");
    // so we can deduce the e wire checking which of the possible e wire if not shown on 3
    let e = e_possibilities
        .iter()
        .find(|wire| !three.contains(**wire))
        .expect("should have found e");
    known_possible_wire(&mut possible_wire, vec!['e'], vec![*e]);
    // so we can deduce the f wire checking which of the possible f wire if not shown on 2
    let f = f_possibilities
        .iter()
        .find(|wire| !two.contains(**wire))
        .expect("should have found f");
    known_possible_wire(&mut possible_wire, vec!['f'], vec![*f]);

    // now we should know the full mapping
    if possible_wire
        .iter()
        .map(|(_, mapping)| mapping.len())
        .sum::<usize>()
        != 7
    {
        panic!("We do not found the mapping {:?}", possible_wire);
    }
    return possible_wire.iter().map(|(k, v)| (*k, v[0])).collect();
}

fn compute_digit_mapping(wire_mapping: &HashMap<char, char>) -> HashMap<String, u32> {
    let with_wire_mapping = |digit_pattern: Vec<char>| {
        let mut pattern: Vec<char> = digit_pattern
            .iter()
            .map(|wire| wire_mapping.get(wire).unwrap())
            .map(|c| *c)
            .collect();
        pattern.sort();
        pattern
            .iter()
            .map(|c| *c)
            .map(String::from)
            .collect::<Vec<String>>()
            .join("")
    };
    let mut digit_mapping: HashMap<String, u32> = HashMap::new();
    digit_mapping.insert(with_wire_mapping(vec!['a', 'b', 'c', 'e', 'f', 'g']), 0);
    digit_mapping.insert(with_wire_mapping(vec!['c', 'f']), 1);
    digit_mapping.insert(with_wire_mapping(vec!['a', 'c', 'd', 'e', 'g']), 2);
    digit_mapping.insert(with_wire_mapping(vec!['a', 'c', 'd', 'f', 'g']), 3);
    digit_mapping.insert(with_wire_mapping(vec!['b', 'c', 'd', 'f']), 4);
    digit_mapping.insert(with_wire_mapping(vec!['a', 'b', 'd', 'f', 'g']), 5);
    digit_mapping.insert(with_wire_mapping(vec!['a', 'b', 'd', 'e', 'f', 'g']), 6);
    digit_mapping.insert(with_wire_mapping(vec!['a', 'c', 'f']), 7);
    digit_mapping.insert(
        with_wire_mapping(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        8,
    );
    digit_mapping.insert(with_wire_mapping(vec!['a', 'b', 'c', 'd', 'f', 'g']), 9);
    return digit_mapping;
}

fn find_digit_from_mapping(digit_mapping: &HashMap<String, u32>, digit_str: &String) -> u32 {
    let mut digit_str: Vec<char> = digit_str.chars().collect();
    digit_str.sort();
    let sorted_digit_str: String = digit_str
        .iter()
        .map(|c| *c)
        .map(String::from)
        .collect::<Vec<String>>()
        .join("");
    *digit_mapping.get(&sorted_digit_str).unwrap()
}

fn find_number_from_pattern(line: &NoteLine) -> u32 {
    let wire_mapping = find_wire_mapping(line);
    let digit_mapping = compute_digit_mapping(&wire_mapping);
    return line
        .digits
        .iter()
        .map(|d| find_digit_from_mapping(&digit_mapping, d))
        .map(|d| format!("{}", d))
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .expect("should be parsable");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn sample_data() -> Vec<NoteLine> {
        vec![
            NoteLine {
                signal_pattern: vec![
                    String::from("be"),
                    String::from("cfbegad"),
                    String::from("cbdgef"),
                    String::from("fgaecd"),
                    String::from("cgeb"),
                    String::from("fdcge"),
                    String::from("agebfd"),
                    String::from("fecdb"),
                    String::from("fabcd"),
                    String::from("edb"),
                ],
                digits: vec![
                    String::from("fdgacbe"),
                    String::from("cefdb"),
                    String::from("cefbgd"),
                    String::from("gcbe"),
                ],
            },
            NoteLine {
                signal_pattern: vec![
                    String::from("edbfga"),
                    String::from("begcd"),
                    String::from("cbg"),
                    String::from("gc"),
                    String::from("gcadebf"),
                    String::from("fbgde"),
                    String::from("acbgfd"),
                    String::from("abcde"),
                    String::from("gfcbed"),
                    String::from("gfec"),
                ],
                digits: vec![
                    String::from("fcgedb"),
                    String::from("cgb"),
                    String::from("dgebacf"),
                    String::from("gc"),
                ],
            },
            NoteLine {
                signal_pattern: vec![
                    String::from("fgaebd"),
                    String::from("cg"),
                    String::from("bdaec"),
                    String::from("gdafb"),
                    String::from("agbcfd"),
                    String::from("gdcbef"),
                    String::from("bgcad"),
                    String::from("gfac"),
                    String::from("gcb"),
                    String::from("cdgabef"),
                ],
                digits: vec![
                    String::from("cg"),
                    String::from("cg"),
                    String::from("fdcagb"),
                    String::from("cbg"),
                ],
            },
            NoteLine {
                signal_pattern: vec![
                    String::from("fbegcd"),
                    String::from("cbd"),
                    String::from("adcefb"),
                    String::from("dageb"),
                    String::from("afcb"),
                    String::from("bc"),
                    String::from("aefdc"),
                    String::from("ecdab"),
                    String::from("fgdeca"),
                    String::from("fcdbega"),
                ],
                digits: vec![
                    String::from("efabcd"),
                    String::from("cedba"),
                    String::from("gadfec"),
                    String::from("cb"),
                ],
            },
            NoteLine {
                signal_pattern: vec![
                    String::from("aecbfdg"),
                    String::from("fbg"),
                    String::from("gf"),
                    String::from("bafeg"),
                    String::from("dbefa"),
                    String::from("fcge"),
                    String::from("gcbea"),
                    String::from("fcaegb"),
                    String::from("dgceab"),
                    String::from("fcbdga"),
                ],
                digits: vec![
                    String::from("gecf"),
                    String::from("egdcabf"),
                    String::from("bgf"),
                    String::from("bfgea"),
                ],
            },
            NoteLine {
                signal_pattern: vec![
                    String::from("fgeab"),
                    String::from("ca"),
                    String::from("afcebg"),
                    String::from("bdacfeg"),
                    String::from("cfaedg"),
                    String::from("gcfdb"),
                    String::from("baec"),
                    String::from("bfadeg"),
                    String::from("bafgc"),
                    String::from("acf"),
                ],
                digits: vec![
                    String::from("gebdcfa"),
                    String::from("ecba"),
                    String::from("ca"),
                    String::from("fadegcb"),
                ],
            },
            NoteLine {
                signal_pattern: vec![
                    String::from("dbcfg"),
                    String::from("fgd"),
                    String::from("bdegcaf"),
                    String::from("fgec"),
                    String::from("aegbdf"),
                    String::from("ecdfab"),
                    String::from("fbedc"),
                    String::from("dacgb"),
                    String::from("gdcebf"),
                    String::from("gf"),
                ],
                digits: vec![
                    String::from("cefg"),
                    String::from("dcbef"),
                    String::from("fcge"),
                    String::from("gbcadfe"),
                ],
            },
            NoteLine {
                signal_pattern: vec![
                    String::from("bdfegc"),
                    String::from("cbegaf"),
                    String::from("gecbf"),
                    String::from("dfcage"),
                    String::from("bdacg"),
                    String::from("ed"),
                    String::from("bedf"),
                    String::from("ced"),
                    String::from("adcbefg"),
                    String::from("gebcd"),
                ],
                digits: vec![
                    String::from("ed"),
                    String::from("bcgafe"),
                    String::from("cdgba"),
                    String::from("cbgef"),
                ],
            },
            NoteLine {
                signal_pattern: vec![
                    String::from("egadfb"),
                    String::from("cdbfeg"),
                    String::from("cegd"),
                    String::from("fecab"),
                    String::from("cgb"),
                    String::from("gbdefca"),
                    String::from("cg"),
                    String::from("fgcdab"),
                    String::from("egfdb"),
                    String::from("bfceg"),
                ],
                digits: vec![
                    String::from("gbdfcae"),
                    String::from("bgc"),
                    String::from("cg"),
                    String::from("cgb"),
                ],
            },
            NoteLine {
                signal_pattern: vec![
                    String::from("gcafb"),
                    String::from("gcf"),
                    String::from("dcaebfg"),
                    String::from("ecagb"),
                    String::from("gf"),
                    String::from("abcdeg"),
                    String::from("gaef"),
                    String::from("cafbge"),
                    String::from("fdbac"),
                    String::from("fegbdc"),
                ],
                digits: vec![
                    String::from("fgae"),
                    String::from("cfgab"),
                    String::from("fg"),
                    String::from("bagce"),
                ],
            },
        ]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ";
        let expected = sample_data();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let data = sample_data();
        let expected = 26;
        assert_eq!(part1(&data).unwrap(), expected);
    }

    #[test_case(1000,  5353  ; "it should find 5353 for exercice sample")]
    #[test_case(0,  8394  ; "it should find 8394 for sample 0")]
    #[test_case(1,  9781  ; "it should find 9781 for sample 1")]
    #[test_case(2,  1197  ; "it should find 1197 for sample 2")]
    #[test_case(3,  9361  ; "it should find 9361 for sample 3")]
    #[test_case(4,  4873  ; "it should find 4873 for sample 4")]
    #[test_case(5,  8418  ; "it should find 8418 for sample 5")]
    #[test_case(6,  4548  ; "it should find 4548 for sample 6")]
    #[test_case(7,  1625  ; "it should find 1625 for sample 7")]
    #[test_case(8,  8717  ; "it should find 8717 for sample 8")]
    #[test_case(9,  4315  ; "it should find 4315 for sample 9")]
    fn find_number_from_pattern_tests(sample_index: usize, expected_digits: u32) {
        let data = sample_data();
        let exercice_sample = NoteLine {
            signal_pattern: vec![
                String::from("acedgfb"),
                String::from("cdfbe"),
                String::from("gcdfa"),
                String::from("fbcad"),
                String::from("dab"),
                String::from("cefabd"),
                String::from("cdfgeb"),
                String::from("eafb"),
                String::from("cagedb"),
                String::from("ab"),
            ],
            digits: vec![
                String::from("cdfeb"),
                String::from("fcadb"),
                String::from("cdfeb"),
                String::from("cdbaf"),
            ],
        };
        let line = if sample_index == 1000 {
            &exercice_sample
        } else {
            &data[sample_index]
        };
        assert_eq!(find_number_from_pattern(line), expected_digits);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let data = sample_data();
        let expected = 61229;
        assert_eq!(part2(&data).unwrap(), expected);
    }
}
