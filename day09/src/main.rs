use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

type Input = Vec<Vec<u32>>;

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
        .map(|file_fragment| {
            file_fragment
                .split("")
                .map(|line_fragment| line_fragment.trim())
                .filter(|line_fragment| !line_fragment.is_empty())
                .map(|line_fragment| line_fragment.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    return Ok(lines);
}

fn part1(data: &Input) -> Result<u32, String> {
    let low_points = get_low_points(data);
    return Ok(low_points.iter().map(|(p, _)| p + 1).sum());
}

fn get_adjacents(data: &Input, x: usize, y: usize) -> Vec<(u32, (usize, usize))> {
    let max_y: isize = (data.len() - 1)
        .try_into()
        .expect("should be able to convert max_y to isize");
    let max_x: isize = (data[0].len() - 1)
        .try_into()
        .expect("should be able to convert max_x to isize");
    let x: isize = x.try_into().expect("should be able to convert x to isize");
    let y: isize = y.try_into().expect("should be able to convert y to isize");
    let adjacent_positions: Vec<(isize, isize)> =
        vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];
    adjacent_positions
        .iter()
        .filter(|(ax, ay)| *ax >= 0 && *ay >= 0 && *ax <= max_x && *ay <= max_y)
        .map(|(ax, ay)| {
            let ax: usize = usize::try_from(*ax).unwrap();
            let ay: usize = usize::try_from(*ay).unwrap();
            (ax, ay)
        })
        .map(|(ax, ay)| (data[ay][ax], (ax, ay)))
        .collect()
}

fn get_low_points(data: &Input) -> Vec<(u32, (usize, usize))> {
    let mut low_points: Vec<(u32, (usize, usize))> = Vec::new();
    for j in 0..data.len() {
        for i in 0..data[j].len() {
            let current = data[j][i];
            let adjacents = get_adjacents(data, i, j);
            if adjacents.iter().all(|(a, _)| *a > current) {
                low_points.push((current, (i, j)));
            }
        }
    }
    return low_points;
}

fn part2(data: &Input) -> Result<usize, String> {
    let low_points = get_low_points(data);
    let mut basins = get_basins_size(data, &low_points);
    basins.sort();
    return Ok(basins.iter().rev().take(3).fold(1, |res, cur| res * cur));
}

fn get_basins_size(data: &Input, low_points: &Vec<(u32, (usize, usize))>) -> Vec<usize> {
    low_points
        .iter()
        .map(|point| explore_basin(data, *point))
        .map(|basin| basin.len())
        .collect()
}

fn explore_basin(data: &Input, point: (u32, (usize, usize))) -> Vec<(u32, (usize, usize))> {
    let mut basin: Vec<(u32, (usize, usize))> = vec![point];
    let mut new: VecDeque<(u32, (usize, usize))> = VecDeque::new();
    new.push_back(point);
    while let Some(current_point) = new.pop_front() {
        let (_, (i, j)) = current_point;
        if basin.iter().all(|(_, (bx, by))| !(*bx == i && *by == j)) {
            basin.push(current_point);
        }
        let adjacents = get_adjacents(data, i, j);
        adjacents
            .iter()
            .filter(|(height, _)| *height < 9)
            .filter(|(_, (ax, ay))| {
                basin
                    .iter()
                    .all(|(_, (bx, by))| !(*bx == *ax && *by == *ay))
            })
            .for_each(|point| {
                new.push_back(*point);
            });
    }
    return basin;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Input {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        let expected = sample_data();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let data = sample_data();
        let expected = 15;
        assert_eq!(part1(&data).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_correctly_low_points() {
        let data = sample_data();
        let expected = vec![(1, (1, 0)), (0, (9, 0)), (5, (2, 2)), (5, (6, 4))];
        assert_eq!(get_low_points(&data), expected);
    }

    #[test]
    fn it_should_explore_correctly_basin_from_low_point_1() {
        let data = sample_data();
        let expected = vec![(1, (1, 0)), (2, (0, 0)), (3, (0, 1))];
        assert_eq!(explore_basin(&data, (1, (1, 0))), expected);
    }

    #[test]
    fn it_should_explore_correctly_basin_from_low_point_2() {
        let data = sample_data();
        let expected = vec![
            (0, (9, 0)),
            (1, (8, 0)),
            (1, (9, 1)),
            (2, (7, 0)),
            (2, (8, 1)),
            (2, (9, 2)),
            (3, (6, 0)),
            (4, (5, 0)),
            (4, (6, 1)),
        ];
        assert_eq!(explore_basin(&data, (0, (9, 0))), expected);
    }

    #[test]
    fn it_should_explore_correctly_basin_from_low_point_3() {
        let data = sample_data();
        let expected = vec![
            (5, (2, 2)),
            (8, (1, 2)),
            (8, (2, 1)),
            (6, (3, 2)),
            (6, (2, 3)),
            (7, (1, 3)),
            (7, (3, 1)),
            (7, (4, 2)),
            (7, (3, 3)),
            (8, (0, 3)),
            (8, (1, 4)),
            (8, (4, 1)),
            (8, (5, 2)),
            (8, (4, 3)),
        ];
        assert_eq!(explore_basin(&data, (5, (2, 2))), expected);
    }

    #[test]
    fn it_should_explore_correctly_basin_from_low_point_4() {
        let data = sample_data();
        let expected = vec![
            (5, (6, 4)),
            (6, (5, 4)),
            (6, (6, 3)),
            (6, (7, 4)),
            (7, (7, 3)),
            (7, (8, 4)),
            (8, (7, 2)),
            (8, (8, 3)),
            (8, (9, 4)),
        ];
        assert_eq!(explore_basin(&data, (5, (6, 4))), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let data = sample_data();
        let expected = 1134;
        assert_eq!(part2(&data).unwrap(), expected);
    }
}
