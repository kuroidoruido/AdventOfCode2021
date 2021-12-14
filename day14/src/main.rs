use core::hash::Hash;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::RwLock;

#[derive(Clone, PartialEq, Debug)]
struct Input {
    template: Vec<char>,
    insertion_map: HashMap<Vec<char>, char>,
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
    let lines: Vec<String> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(String::from)
        .collect();
    let mut it = lines.iter();

    let template: Vec<char> = it
        .next()
        .expect("should have at least one line")
        .chars()
        .collect();

    let mut insertion_map: HashMap<Vec<char>, char> = HashMap::new();
    while let Some(mapping) = it.next() {
        let (from, to): (&str, &str) = mapping.split_once(" -> ").expect("should be valid mapping");
        insertion_map.insert(
            from.chars().collect(),
            to.chars().nth(0).expect("should have one char"),
        );
    }

    return Ok(Input {
        template,
        insertion_map,
    });
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    let occurences = compute_insertions(&input.template, &input.insertion_map, 10);
    let max = occurences
        .iter()
        .map(|(_, count)| count)
        .max()
        .expect("should have one element");
    let min = occurences
        .iter()
        .map(|(_, count)| count)
        .min()
        .expect("should have one element");
    return Ok(max - min);
}

lazy_static! {
    static ref CACHE: RwLock<HashMap<(Vec<char>, usize), HashMap<char, usize>>> =
        RwLock::new(HashMap::new());
}

fn compute_insertions(
    template: &Vec<char>,
    insertion_map: &HashMap<Vec<char>, char>,
    iteration: usize,
) -> HashMap<char, usize> {
    {
        let cache = CACHE.read().unwrap();
        if let Some(previous_res) = cache.get(&(template.clone(), iteration)) {
            return previous_res.clone();
        }
    }

    if iteration == 0 {
        count_occurences(&template.to_vec())
    } else {
        let res = template
            .iter()
            .tuple_windows::<(&char, &char)>()
            .enumerate()
            .map(|(index, (a, b))| {
                let new_template = apply_insertion(*a, *b, insertion_map);
                if index == 0 {
                    compute_insertions(&new_template, insertion_map, iteration - 1)
                } else {
                    let mut count = compute_insertions(&new_template, insertion_map, iteration - 1);
                    count.insert(*a, count.get(a).unwrap() - 1);
                    count
                }
            })
            .reduce(|mut acc, cur| {
                cur.iter().for_each(|(c, count)| {
                    if let Some(acc_count) = acc.get(c) {
                        acc.insert(*c, acc_count + *count);
                    } else {
                        acc.insert(*c, *count);
                    }
                });
                return acc;
            })
            .expect("should be able to reduce");
        let mut cache = CACHE.write().unwrap();
        cache.insert((template.clone(), iteration), res.clone());
        return res;
    }
}

fn apply_insertion(before: char, after: char, map: &HashMap<Vec<char>, char>) -> Vec<char> {
    vec![
        before,
        *map.get(&vec![before, after])
            .expect("should have the mapping"),
        after,
    ]
}

fn count_occurences<T: Clone + Eq + Hash>(v: &Vec<T>) -> HashMap<T, usize> {
    let mut count: HashMap<T, usize> = HashMap::new();
    v.iter().for_each(|x| {
        if let Some(actual) = count.get(x) {
            count.insert(x.clone(), actual + 1);
        } else {
            count.insert(x.clone(), 1);
        }
    });
    return count;
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let occurences = compute_insertions(&input.template, &input.insertion_map, 40);
    let max = occurences
        .iter()
        .map(|(_, count)| count)
        .max()
        .expect("should have one element");
    let min = occurences
        .iter()
        .map(|(_, count)| count)
        .min()
        .expect("should have one element");
    return Ok(max - min);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        let mut insertion_map: HashMap<Vec<char>, char> = HashMap::new();
        insertion_map.insert(vec!['C', 'H'], 'B');
        insertion_map.insert(vec!['H', 'H'], 'N');
        insertion_map.insert(vec!['C', 'B'], 'H');
        insertion_map.insert(vec!['N', 'H'], 'C');
        insertion_map.insert(vec!['H', 'B'], 'C');
        insertion_map.insert(vec!['H', 'C'], 'B');
        insertion_map.insert(vec!['H', 'N'], 'C');
        insertion_map.insert(vec!['N', 'N'], 'C');
        insertion_map.insert(vec!['B', 'H'], 'H');
        insertion_map.insert(vec!['N', 'C'], 'B');
        insertion_map.insert(vec!['N', 'B'], 'B');
        insertion_map.insert(vec!['B', 'N'], 'B');
        insertion_map.insert(vec!['B', 'B'], 'N');
        insertion_map.insert(vec!['B', 'C'], 'B');
        insertion_map.insert(vec!['C', 'C'], 'N');
        insertion_map.insert(vec!['C', 'N'], 'C');
        Input {
            template: vec!['N', 'N', 'C', 'B'],
            insertion_map,
        }
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_insertions_correctly_1() {
        let input = sample_input();
        let mut expected: HashMap<char, usize> = HashMap::new();
        expected.insert('N', 2);
        expected.insert('B', 2);
        expected.insert('C', 2);
        expected.insert('H', 1);
        assert_eq!(
            compute_insertions(&input.template, &input.insertion_map, 1),
            expected
        );
    }

    #[test]
    fn it_should_compute_insertions_correctly_2() {
        let input = sample_input();
        let mut expected: HashMap<char, usize> = HashMap::new();
        expected.insert('N', 2);
        expected.insert('B', 6);
        expected.insert('C', 4);
        expected.insert('H', 1);
        assert_eq!(
            compute_insertions(&input.template, &input.insertion_map, 2),
            expected
        );
    }

    #[test]
    fn it_should_compute_insertions_correctly_3() {
        let input = sample_input();
        let mut expected: HashMap<char, usize> = HashMap::new();
        expected.insert('N', 5);
        expected.insert('B', 11);
        expected.insert('C', 5);
        expected.insert('H', 4);
        assert_eq!(
            compute_insertions(&input.template, &input.insertion_map, 3),
            expected
        );
    }

    #[test]
    fn it_should_compute_insertions_correctly_4() {
        let input = sample_input();
        let mut expected: HashMap<char, usize> = HashMap::new();
        expected.insert('N', 11);
        expected.insert('B', 23);
        expected.insert('C', 10);
        expected.insert('H', 5);
        assert_eq!(
            compute_insertions(&input.template, &input.insertion_map, 4),
            expected
        );
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = 1588;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 2188189693529;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
