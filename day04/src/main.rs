use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq)]
enum BoardNumberState {
    Idle,
    Marked,
}

type BoardRow = Vec<(u32, BoardNumberState)>;
type Board = Vec<BoardRow>;

#[derive(Clone, Debug, PartialEq)]
struct Game {
    random_numbers: Vec<u32>,
    boards: Vec<Board>,
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

fn parse_data(input: String) -> Result<Game, String> {
    let mut lines_it = input.split("\n").map(|file_fragment| file_fragment.trim());
    // random numbers
    let random_numbers = lines_it.next().expect("file should have at least one line");
    let random_numbers = random_numbers
        .split(",")
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.parse::<u32>()
                .expect(format!("should be a number: {}", x).as_str())
        })
        .collect();
    // boards
    let mut boards: Vec<Board> = Vec::new();
    let mut board: Board = Vec::new();
    while let Some(line) = lines_it.next() {
        if line.is_empty() {
            if !board.is_empty() {
                boards.push(board);
            }
            board = Vec::new();
        } else {
            board.push(
                line.split(" ")
                    .map(str::trim)
                    .filter(|x| !x.is_empty())
                    .map(|x| {
                        x.parse::<u32>()
                            .expect(format!("should be a number: {}", x).as_str())
                    })
                    .map(|x| (x, BoardNumberState::Idle))
                    .collect(),
            );
        }
    }
    if !board.is_empty() {
        boards.push(board);
    }

    return Ok(Game {
        random_numbers,
        boards,
    });
}

fn part1(data: &Game) -> Result<u32, String> {
    let mut random_numbers_it = data.random_numbers.iter();
    let mut boards_state = data.boards.clone();
    while let Some(current_number) = random_numbers_it.next() {
        boards_state = boards_state
            .iter()
            .map(|board| mark_number(&board, *current_number))
            .collect();
        if boards_state.iter().any(is_winner_board) {
            let winner = boards_state.iter().find(is_winner_board_find).unwrap();
            let sum: u32 = winner
                .iter()
                .flat_map(|row| {
                    row.iter()
                        .filter(|(_, state)| *state == BoardNumberState::Idle)
                        .map(|x| *x)
                        .collect::<BoardRow>()
                })
                .map(|(n, _)| n)
                .sum();
            return Ok(current_number * sum);
        }
    }
    return Err(String::from("No winner board found"));
}

fn part2(data: &Game) -> Result<u32, String> {
    let mut random_numbers_it = data.random_numbers.iter();
    let mut boards_state = data.boards.clone();
    while let Some(current_number) = random_numbers_it.next() {
        boards_state = boards_state
            .iter()
            .map(|board| mark_number(&board, *current_number))
            .collect();
        if boards_state.iter().any(is_winner_board) {
            if boards_state.len() > 1 {
                boards_state = boards_state
                    .iter()
                    .filter(|board| !is_winner_board(board))
                    .map(|x| x.clone())
                    .collect();
            } else {
                let winner = boards_state.iter().find(is_winner_board_find).unwrap();
                let sum: u32 = winner
                    .iter()
                    .flat_map(|row| {
                        row.iter()
                            .filter(|(_, state)| *state == BoardNumberState::Idle)
                            .map(|x| *x)
                            .collect::<BoardRow>()
                    })
                    .map(|(n, _)| n)
                    .sum();
                return Ok(current_number * sum);
            }
        }
    }
    return Err(String::from("No winner board found"));
}

fn is_winner_row(row: &BoardRow) -> bool {
    row.iter()
        .all(|(_, state)| *state == BoardNumberState::Marked)
}
fn is_winner_board(board: &Board) -> bool {
    return board.iter().any(is_winner_row)
        || convert_to_column_matrix(&board, board[0].len())
            .iter()
            .any(is_winner_row);
}
fn is_winner_board_find(board: &&Board) -> bool {
    is_winner_board(*board)
}

fn convert_to_column_matrix<T: Clone>(matrix: &Vec<Vec<T>>, column_count: usize) -> Vec<Vec<T>> {
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

fn mark_number(board: &Board, current_number: u32) -> Board {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|(n, state)| {
                    if *n == current_number {
                        (*n, BoardNumberState::Marked)
                    } else {
                        (*n, *state)
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::BoardNumberState::*;
    use super::*;

    #[test]
    fn it_should_parse_correctly() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7";
        let expected = Game {
            random_numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                vec![
                    vec![(22, Idle), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
                    vec![(8, Idle), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
                    vec![(21, Idle), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
                    vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
                    vec![(1, Idle), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
                ],
                vec![
                    vec![(3, Idle), (15, Idle), (0, Idle), (2, Idle), (22, Idle)],
                    vec![(9, Idle), (18, Idle), (13, Idle), (17, Idle), (5, Idle)],
                    vec![(19, Idle), (8, Idle), (7, Idle), (25, Idle), (23, Idle)],
                    vec![(20, Idle), (11, Idle), (10, Idle), (24, Idle), (4, Idle)],
                    vec![(14, Idle), (21, Idle), (16, Idle), (12, Idle), (6, Idle)],
                ],
                vec![
                    vec![(14, Idle), (21, Idle), (17, Idle), (24, Idle), (4, Idle)],
                    vec![(10, Idle), (16, Idle), (15, Idle), (9, Idle), (19, Idle)],
                    vec![(18, Idle), (8, Idle), (23, Idle), (26, Idle), (20, Idle)],
                    vec![(22, Idle), (11, Idle), (13, Idle), (6, Idle), (5, Idle)],
                    vec![(2, Idle), (0, Idle), (12, Idle), (3, Idle), (7, Idle)],
                ],
            ],
        };
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_winning_board_correctly() {
        let board1: Board = vec![
            vec![(22, Marked), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
            vec![(8, Marked), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
            vec![(21, Marked), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Marked), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Marked), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        let board2: Board = vec![
            vec![
                (22, Marked),
                (13, Marked),
                (17, Marked),
                (11, Marked),
                (0, Marked),
            ],
            vec![(8, Idle), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
            vec![(21, Idle), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Idle), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        assert_eq!(is_winner_board(&board1), true);
        assert_eq!(is_winner_board(&board2), true);
    }

    #[test]
    fn it_should_compute_loosing_board_correctly() {
        let board1: Board = vec![
            vec![(22, Marked), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
            vec![(8, Marked), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
            vec![(21, Marked), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Marked), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        let board2: Board = vec![
            vec![(22, Idle), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
            vec![
                (8, Marked),
                (2, Marked),
                (23, Marked),
                (4, Idle),
                (24, Marked),
            ],
            vec![(21, Idle), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Idle), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        assert_eq!(is_winner_board(&board1), false);
        assert_eq!(is_winner_board(&board2), false);
    }

    #[test]
    fn it_should_mark_number_on_board_correctly_1() {
        let board: Board = vec![
            vec![(22, Idle), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
            vec![(8, Idle), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
            vec![(21, Idle), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Idle), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        let expected: Board = vec![
            vec![(22, Idle), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
            vec![(8, Idle), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
            vec![(21, Idle), (9, Marked), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Idle), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        assert_eq!(mark_number(&board, 9), expected);
    }
    #[test]
    fn it_should_mark_number_on_board_correctly_2() {
        let board: Board = vec![
            vec![(22, Marked), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
            vec![(8, Marked), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
            vec![(21, Marked), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Marked), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        let expected: Board = vec![
            vec![(22, Marked), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
            vec![(8, Marked), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
            vec![(21, Marked), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Marked), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Marked), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        assert_eq!(mark_number(&board, 6), expected);
    }
    #[test]
    fn it_should_mark_number_on_board_correctly_3() {
        let board: Board = vec![
            vec![(22, Idle), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
            vec![
                (8, Marked),
                (2, Marked),
                (23, Marked),
                (4, Idle),
                (24, Marked),
            ],
            vec![(21, Idle), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Idle), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        let expected: Board = vec![
            vec![(22, Idle), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
            vec![
                (8, Marked),
                (2, Marked),
                (23, Marked),
                (4, Marked),
                (24, Marked),
            ],
            vec![(21, Idle), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
            vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
            vec![(1, Idle), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
        ];
        assert_eq!(mark_number(&board, 4), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let data = Game {
            random_numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                vec![
                    vec![(22, Idle), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
                    vec![(8, Idle), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
                    vec![(21, Idle), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
                    vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
                    vec![(1, Idle), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
                ],
                vec![
                    vec![(3, Idle), (15, Idle), (0, Idle), (2, Idle), (22, Idle)],
                    vec![(9, Idle), (18, Idle), (13, Idle), (17, Idle), (5, Idle)],
                    vec![(19, Idle), (8, Idle), (7, Idle), (25, Idle), (23, Idle)],
                    vec![(20, Idle), (11, Idle), (10, Idle), (24, Idle), (4, Idle)],
                    vec![(14, Idle), (21, Idle), (16, Idle), (12, Idle), (6, Idle)],
                ],
                vec![
                    vec![(14, Idle), (21, Idle), (17, Idle), (24, Idle), (4, Idle)],
                    vec![(10, Idle), (16, Idle), (15, Idle), (9, Idle), (19, Idle)],
                    vec![(18, Idle), (8, Idle), (23, Idle), (26, Idle), (20, Idle)],
                    vec![(22, Idle), (11, Idle), (13, Idle), (6, Idle), (5, Idle)],
                    vec![(2, Idle), (0, Idle), (12, Idle), (3, Idle), (7, Idle)],
                ],
            ],
        };
        let expected = 4512;
        assert_eq!(part1(&data).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let data = Game {
            random_numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                vec![
                    vec![(22, Idle), (13, Idle), (17, Idle), (11, Idle), (0, Idle)],
                    vec![(8, Idle), (2, Idle), (23, Idle), (4, Idle), (24, Idle)],
                    vec![(21, Idle), (9, Idle), (14, Idle), (16, Idle), (7, Idle)],
                    vec![(6, Idle), (10, Idle), (3, Idle), (18, Idle), (5, Idle)],
                    vec![(1, Idle), (12, Idle), (20, Idle), (15, Idle), (19, Idle)],
                ],
                vec![
                    vec![(3, Idle), (15, Idle), (0, Idle), (2, Idle), (22, Idle)],
                    vec![(9, Idle), (18, Idle), (13, Idle), (17, Idle), (5, Idle)],
                    vec![(19, Idle), (8, Idle), (7, Idle), (25, Idle), (23, Idle)],
                    vec![(20, Idle), (11, Idle), (10, Idle), (24, Idle), (4, Idle)],
                    vec![(14, Idle), (21, Idle), (16, Idle), (12, Idle), (6, Idle)],
                ],
                vec![
                    vec![(14, Idle), (21, Idle), (17, Idle), (24, Idle), (4, Idle)],
                    vec![(10, Idle), (16, Idle), (15, Idle), (9, Idle), (19, Idle)],
                    vec![(18, Idle), (8, Idle), (23, Idle), (26, Idle), (20, Idle)],
                    vec![(22, Idle), (11, Idle), (13, Idle), (6, Idle), (5, Idle)],
                    vec![(2, Idle), (0, Idle), (12, Idle), (3, Idle), (7, Idle)],
                ],
            ],
        };
        let expected = 1924;
        assert_eq!(part2(&data).unwrap(), expected);
    }
}
