use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

type Input = Vec<String>;
type Part1Output = u64;
type Part2Output = u64;

#[derive(Clone, PartialEq, Debug)]
enum LineStatus {
    Valid,
    Incomplete(Vec<char>),
    Corrupted(char),
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

fn parse_data(input: String) -> Result<Input, String> {
    let lines: Input = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(String::from)
        .collect();
    return Ok(lines);
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    return Ok(input
        .iter()
        .map(get_line_status)
        .filter(is_corrupted)
        .map(get_corrupted_char)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(corrupted_char_to_score)
        .sum());
}

fn is_corrupted(status: &LineStatus) -> bool {
    match status {
        LineStatus::Corrupted(_) => true,
        _ => false,
    }
}

fn get_corrupted_char(status: LineStatus) -> Option<char> {
    match status {
        LineStatus::Corrupted(c) => Some(c),
        _ => None,
    }
}

fn is_incomplete(status: &LineStatus) -> bool {
    match status {
        LineStatus::Incomplete(_) => true,
        _ => false,
    }
}

fn get_incomplete_chars(status: LineStatus) -> Option<Vec<char>> {
    match status {
        LineStatus::Incomplete(cc) => Some(cc),
        _ => None,
    }
}

fn get_line_status(line: &String) -> LineStatus {
    fn is_opening_char(c: char) -> bool {
        match c {
            '(' | '[' | '<' | '{' => true,
            _ => false,
        }
    }
    fn is_matching_chars(opening: char, closing: char) -> bool {
        match opening {
            '(' => closing == ')',
            '[' => closing == ']',
            '<' => closing == '>',
            '{' => closing == '}',
            _ => false,
        }
    }

    let mut stack: VecDeque<char> = VecDeque::new();

    for c in line.chars() {
        if is_opening_char(c) {
            stack.push_back(c);
        } else if is_matching_chars(*stack.back().expect("should have one element"), c) {
            stack.pop_back();
        } else {
            return LineStatus::Corrupted(c);
        }
    }

    return if stack.is_empty() {
        LineStatus::Valid
    } else {
        LineStatus::Incomplete(stack.into())
    };
}

fn corrupted_char_to_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn missing_char_to_score(c: &char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn incomplete_line_score(missing_chars: Vec<char>) -> u64 {
    missing_chars
        .iter()
        .map(missing_char_to_score)
        .fold(0, |total, cur| total * 5 + cur)
}

fn convert_opening_to_closing(opening: Vec<char>) -> Vec<char> {
    fn open_to_close(c: &char) -> Option<char> {
        match c {
            '(' => Some(')'),
            '[' => Some(']'),
            '<' => Some('>'),
            '{' => Some('}'),
            _ => None,
        }
    }
    opening
        .iter()
        .rev()
        .map(open_to_close)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let mut line_scores: Vec<u64> = input
        .iter()
        .map(get_line_status)
        .filter(is_incomplete)
        .map(get_incomplete_chars)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(convert_opening_to_closing)
        .map(incomplete_line_score)
        .collect();
    line_scores.sort();
    return Ok(line_scores[line_scores.len() / 2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
        ]
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_detect_correcty_valid_chunk() {
        assert_eq!(get_line_status(&String::from("()")), LineStatus::Valid);
        assert_eq!(get_line_status(&String::from("[]")), LineStatus::Valid);
        assert_eq!(get_line_status(&String::from("([])")), LineStatus::Valid);
        assert_eq!(
            get_line_status(&String::from("{()()()}")),
            LineStatus::Valid
        );
        assert_eq!(
            get_line_status(&String::from("<([{}])>")),
            LineStatus::Valid
        );
        assert_eq!(
            get_line_status(&String::from("[<>({}){}[([])<>]]")),
            LineStatus::Valid
        );
        assert_eq!(
            get_line_status(&String::from("(((((((((())))))))))")),
            LineStatus::Valid
        );
    }

    #[test]
    fn it_should_detect_correcty_corrupted_chunk() {
        assert_eq!(
            get_line_status(&String::from("(]")),
            LineStatus::Corrupted(']')
        );
        assert_eq!(
            get_line_status(&String::from("{()()()>")),
            LineStatus::Corrupted('>')
        );
        assert_eq!(
            get_line_status(&String::from("(((()))}")),
            LineStatus::Corrupted('}')
        );
        assert_eq!(
            get_line_status(&String::from("<([]){()}[{}])")),
            LineStatus::Corrupted(')')
        );
        assert_eq!(
            get_line_status(&String::from("{([(<{}[<>[]}>{[]{[(<()>")),
            LineStatus::Corrupted('}')
        );
        assert_eq!(
            get_line_status(&String::from("[[<[([]))<([[{}[[()]]]")),
            LineStatus::Corrupted(')')
        );
        assert_eq!(
            get_line_status(&String::from("[{[{({}]{}}([{[{{{}}([]")),
            LineStatus::Corrupted(']')
        );
        assert_eq!(
            get_line_status(&String::from("[<(<(<(<{}))><([]([]()")),
            LineStatus::Corrupted(')')
        );
        assert_eq!(
            get_line_status(&String::from("<{([([[(<>()){}]>(<<{{")),
            LineStatus::Corrupted('>')
        );
    }

    #[test]
    fn it_should_detect_correcty_incomplete_chunk() {
        assert_eq!(
            get_line_status(&String::from("[({(<(())[]>[[{[]{<()<>>")),
            LineStatus::Incomplete("[({([[{{".chars().collect::<Vec<char>>())
        );
        assert_eq!(
            get_line_status(&String::from("[(()[<>])]({[<{<<[]>>(")),
            LineStatus::Incomplete("({[<{(".chars().collect::<Vec<char>>())
        );
        assert_eq!(
            get_line_status(&String::from("(((({<>}<{<{<>}{[]{[]{}")),
            LineStatus::Incomplete("((((<{<{{".chars().collect::<Vec<char>>())
        );
        assert_eq!(
            get_line_status(&String::from("{<[[]]>}<{[{[{[]{()[[[]")),
            LineStatus::Incomplete("<{[{[{{[[".chars().collect::<Vec<char>>())
        );
        assert_eq!(
            get_line_status(&String::from("<{([{{}}[<[[[<>{}]]]>[]]")),
            LineStatus::Incomplete("<{([".chars().collect::<Vec<char>>())
        );
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = 26397;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_convert_to_closing_correctly() {
        assert_eq!(
            convert_opening_to_closing("[({([[{{".chars().collect()),
            "}}]])})]".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            convert_opening_to_closing("({[<{(".chars().collect()),
            ")}>]})".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            convert_opening_to_closing("((((<{<{{".chars().collect()),
            "}}>}>))))".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            convert_opening_to_closing("<{[{[{{[[".chars().collect()),
            "]]}}]}]}>".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            convert_opening_to_closing("<{([".chars().collect()),
            "])}>".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn it_should_compute_correctly_incomplete_line_score() {
        let missing = "}}]])})]".chars().collect::<Vec<char>>();
        assert_eq!(incomplete_line_score(missing), 288957);
        let missing = ")}>]})".chars().collect::<Vec<char>>();
        assert_eq!(incomplete_line_score(missing), 5566);
        let missing = "}}>}>))))".chars().collect::<Vec<char>>();
        assert_eq!(incomplete_line_score(missing), 1480781);
        let missing = "]]}}]}]}>".chars().collect::<Vec<char>>();
        assert_eq!(incomplete_line_score(missing), 995444);
        let missing = "])}>".chars().collect::<Vec<char>>();
        assert_eq!(incomplete_line_score(missing), 294);
    }

    #[test]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 288957;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
