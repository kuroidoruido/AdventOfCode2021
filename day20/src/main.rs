use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Pixel {
    Light,
    Dark,
}

impl Pixel {
    const fn is_light(&self) -> bool {
        match *self {
            Pixel::Light => true,
            _ => false,
        }
    }
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '#' => Ok(Pixel::Light),
            '.' => Ok(Pixel::Dark),
            _ => Err(format!("Not a valid char for Pixel: {}", c)),
        }
    }
}

impl core::fmt::Debug for Pixel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Pixel::Light => write!(f, "█"),
            Pixel::Dark => write!(f, "░"),
        }
    }
}

type Algorithm = Vec<Pixel>;
type Image = Vec<Vec<Pixel>>;

fn print_image(img: &Image) {
    for j in 0..img.len() {
        for i in 0..img[0].len() {
            print!("{:?}", img[j][i]);
        }
        println!("");
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Input {
    algorithm: Algorithm,
    image: Image,
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
    let mut lines = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty());

    let algorithm = lines
        .next()
        .unwrap()
        .chars()
        .map(Pixel::from_char)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();
    let image = lines
        .map(|row| {
            row.chars()
                .map(Pixel::from_char)
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect::<Vec<Pixel>>()
        })
        .collect();
    return Ok(Input { algorithm, image });
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    let image = enhance_image(input, 2);
    return Ok(count_lit_pixels(&image));
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let image = enhance_image(input, 50);
    return Ok(count_lit_pixels(&image));
}

fn enhance_image(input: &Input, iteration_count: usize) -> Image {
    fn get_pixel_around(img: &Image, x: isize, y: isize, empty: Pixel) -> Vec<Pixel> {
        let positions = vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ];
        positions
            .iter()
            .map(|(xi, yi)| {
                if *xi < 0
                    || *yi < 0
                    || *xi >= (img.len()).try_into().unwrap()
                    || *yi >= (img[0].len()).try_into().unwrap()
                {
                    return empty.clone();
                }
                let x: usize = (*xi).try_into().unwrap();
                let y: usize = (*yi).try_into().unwrap();
                img.get(x).unwrap().get(y).unwrap().clone()
            })
            .collect()
    }

    let algo = input.algorithm.clone();
    let mut current_image = input.image.clone();
    for iteration in 0..iteration_count {
        let mut new_image: Image =
            vec![vec![Pixel::Dark; current_image[0].len() + 2]; current_image.len() + 2];
        let empty_pixel = if iteration % 2 == 0 {
            Pixel::Dark
        } else {
            algo[0]
        };
        for i in 0..new_image.len() {
            for j in 0..new_image[0].len() {
                let ii: isize = i.try_into().unwrap();
                let ji: isize = j.try_into().unwrap();
                let original_pixel_seq =
                    get_pixel_around(&current_image, ii - 1, ji - 1, empty_pixel);
                let algo_result_position = pixel_vec_to_number(&original_pixel_seq);
                new_image[i][j] = algo[algo_result_position];
            }
        }
        current_image = new_image;
    }
    return current_image;
}

fn pixel_vec_to_number(pixels: &Vec<Pixel>) -> usize {
    let binary: String = pixels
        .iter()
        .map(|p| match p {
            Pixel::Light => "1",
            Pixel::Dark => "0",
        })
        .collect::<Vec<&str>>()
        .join("");
    usize::from_str_radix(&binary.as_str(), 2).expect("should be able to convert to number")
}

fn count_lit_pixels(image: &Image) -> usize {
    image
        .iter()
        .flat_map(|row| row)
        .cloned()
        .filter(Pixel::is_light)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
        
        #..#.
        #....
        ##..#
        ..#..
        ..###";
        parse_data(input.to_string()).unwrap()
    }

    #[test]
    fn it_should_convert_correctly_pixels_to_number() {
        let input: Vec<Pixel> = vec![
            Pixel::Dark,
            Pixel::Dark,
            Pixel::Dark,
            Pixel::Light,
            Pixel::Dark,
            Pixel::Dark,
            Pixel::Dark,
            Pixel::Light,
            Pixel::Dark,
        ];
        assert_eq!(pixel_vec_to_number(&input), 34);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = 35;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly_with_test() {
        let input = parse_data(read_input("input_test.txt").unwrap()).unwrap();
        let expected = 5326;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 3351;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
