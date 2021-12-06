use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Fish = u8;

fn main() -> std::io::Result<()> {
    let input1 = read_input("input1.txt").expect("An error occurred when reading input1.txt");
    let data1 = parse_data(input1).expect("An error occurred when parsing input1.txt");

    println!("Part 1: {:?}", part1(&data1, 80));
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

fn parse_data(input: String) -> Result<Vec<Fish>, String> {
    let (fishes, _) = input
        .split_once("\n")
        .expect("should have at least one line");
    return Ok(fishes
        .split(',')
        .map(str::trim)
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| file_fragment.parse::<Fish>().unwrap())
        .collect());
}

fn part1(data: &Vec<Fish>, run_days_count: usize) -> Result<u64, String> {
    let mut fishes: HashMap<u8, u64> = HashMap::new();
    (0..9).for_each(|day| {
        fishes.insert(
            day,
            data.iter().filter(|fish| **fish == day).map(|_| 1).sum(),
        );
    });
    for _ in 0..run_days_count {
        let mut new_fishes: HashMap<u8, u64> = HashMap::new();
        for day in 0..9 {
            let fish_count = *(fishes.get(&day).or(Some(&0)).unwrap());
            if day == 0 {
                let new_fish_count_6 = *(new_fishes.get(&6).or(Some(&0)).unwrap());
                let new_fish_count_8 = *(new_fishes.get(&8).or(Some(&0)).unwrap());
                new_fishes.insert(6, new_fish_count_6 + fish_count);
                new_fishes.insert(8, new_fish_count_8 + fish_count);
            } else {
                let new_fish_count = *(new_fishes.get(&(day - 1)).or(Some(&0)).unwrap());
                new_fishes.insert(day - 1, new_fish_count + fish_count);
            }
        }
        fishes = new_fishes;
    }
    return Ok(fishes.iter().map(|(_, count)| count).sum());
}

fn part2(data: &Vec<Fish>) -> Result<u64, String> {
    return part1(data, 256);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<Fish> {
        vec![3, 4, 3, 1, 2]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "3,4,3,1,2\n";
        let expected = sample_data();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_18_correctly() {
        let data = sample_data();
        let expected = 26;
        assert_eq!(part1(&data, 18).unwrap(), expected);
    }
    #[test]
    fn it_should_compute_part1_80_correctly() {
        let data = sample_data();
        let expected = 5934;
        assert_eq!(part1(&data, 80).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let data = sample_data();
        let expected = 26984457539; // 26 984 457 539
        assert_eq!(part2(&data).unwrap(), expected);
    }
}
