use std::fs::File;
use std::io::prelude::*;
use std::iter::Peekable;
use std::str::Chars;
use trace::trace;

trace::init_depth_var!();

type NodeId = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
enum SnailfishNumber {
    Number(u32),
    Pair(NodeId, NodeId),
}

impl SnailfishNumber {
    fn get_num(self) -> Option<u32> {
        match self {
            SnailfishNumber::Number(n) => Some(n),
            _ => None,
        }
    }
    fn is_pair(self) -> bool {
        match self {
            SnailfishNumber::Pair(_, _) => true,
            _ => false,
        }
    }
    fn get_left(self) -> Option<usize> {
        match self {
            SnailfishNumber::Pair(left, _) => Some(left),
            _ => None,
        }
    }
    fn get_right(self) -> Option<usize> {
        match self {
            SnailfishNumber::Pair(_, right) => Some(right),
            _ => None,
        }
    }
    fn is_number(self) -> bool {
        !self.is_pair()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SnailfishTree {
    nodes: Vec<SnailfishNumber>,
    parent: Vec<Option<usize>>,
}

impl SnailfishTree {
    fn add_number(&mut self, n: SnailfishNumber) -> usize {
        let node_id = self.nodes.len();
        self.nodes.push(n);
        self.parent.push(None);
        node_id
    }
    fn replace(&mut self, node_id: usize, n: SnailfishNumber) {
        self.nodes[node_id] = n;
    }
    fn set_parent(&mut self, parent_node_id: Option<usize>, node_id: usize) {
        self.parent[node_id] = parent_node_id;
    }
    fn get_root_id(&self) -> Option<NodeId> {
        Some(
            self.parent
                .iter()
                .enumerate()
                .find(|(_, parent)| parent.is_none())
                .unwrap()
                .0,
        )
    }
    #[trace]
    fn get_depth(&self, node_id: usize) -> usize {
        match self.parent[node_id] {
            None => 1,
            Some(parent_id) => 1 + self.get_depth(parent_id),
        }
    }
    #[trace]
    fn get_first_right_child_number(&self, node_id: usize) -> usize {
        match self.nodes[node_id] {
            SnailfishNumber::Pair(_, right_id) => self.get_first_right_child_number(right_id),
            SnailfishNumber::Number(_) => node_id,
        }
    }
    #[trace]
    fn get_first_left_child_number(&self, node_id: usize) -> usize {
        match self.nodes[node_id] {
            SnailfishNumber::Pair(left_id, _) => self.get_first_left_child_number(left_id),
            SnailfishNumber::Number(_) => node_id,
        }
    }
    #[trace]
    fn get_first_left_parent_number(&self, node_id: usize) -> Option<usize> {
        let parent_id = self.parent[node_id]?;
        let parent = self.nodes[parent_id];
        if let SnailfishNumber::Pair(left_id, _) = parent {
            if left_id == node_id {
                return self.get_first_left_parent_number(parent_id);
            } else {
                match self.nodes[left_id] {
                    SnailfishNumber::Number(_) => return Some(left_id),
                    SnailfishNumber::Pair(_, _) => {
                        let right = self.get_first_right_child_number(left_id);
                        if right == node_id {
                            return None;
                        } else {
                            return Some(right);
                        }
                    }
                }
            }
        } else {
            return None;
        }
    }
    #[trace]
    fn get_first_right_parent_number(&self, node_id: usize) -> Option<usize> {
        let parent_id = self.parent[node_id]?;
        let parent = self.nodes[parent_id];
        if let SnailfishNumber::Pair(_, right_id) = parent {
            if right_id == node_id {
                return self.get_first_right_parent_number(parent_id);
            } else {
                match self.nodes[right_id] {
                    SnailfishNumber::Number(_) => return Some(right_id),
                    SnailfishNumber::Pair(_, _) => {
                        let left = self.get_first_left_child_number(right_id);
                        if left == node_id {
                            return None;
                        } else {
                            return Some(left);
                        }
                    }
                }
            }
        } else {
            return None;
        }
    }
    #[trace]
    fn add_and_reduce(&self, other: &Self) -> Self {
        let mut res = SnailfishTree {
            nodes: self.nodes.clone(),
            parent: self.parent.clone(),
        };
        // add self and other
        {
            let self_parent = self.get_root_id();
            let other_parent = other.get_root_id();
            let other_id_increment = self.nodes.len();
            let other_parent = other_parent.unwrap() + other_id_increment;
            let mut other_nodes = other.nodes.clone();
            res.nodes.append(&mut other_nodes);
            other.parent.iter().for_each(|p| match p {
                Some(id) => res.parent.push(Some(id + other_id_increment)),
                None => res.parent.push(None),
            });
            let new_root = SnailfishNumber::Pair(self_parent.unwrap(), other_parent);
            let new_root_id = res.add_number(new_root);
            res.set_parent(Some(new_root_id), self_parent.unwrap());
            res.set_parent(Some(new_root_id), other_parent);
        }
        // reduce
        {
            let mut has_made_something = false;
            loop {
                // check for explode
                if let Some(will_reduce) = (0..res.nodes.len())
                    .find(|node_id| res.nodes[*node_id].is_pair() && res.get_depth(*node_id) >= 4)
                {
                    has_made_something = true;
                    println!(
                        "YAOURT will explode {} => {:?} (parent: {:?})",
                        will_reduce, res.nodes[will_reduce], res.parent[will_reduce]
                    );
                    let left_child_number_id = res.nodes[will_reduce].get_left().unwrap();
                    let right_child_number_id = res.nodes[will_reduce].get_right().unwrap();
                    if let Some(first_left_number) = res.get_first_left_parent_number(will_reduce) {
                        let left_child_number = res.nodes[left_child_number_id];
                        res.nodes[first_left_number] = SnailfishNumber::Number(
                            res.nodes[first_left_number].get_num().unwrap()
                                + left_child_number.get_num().unwrap(),
                        );
                    } else {
                        res.nodes[will_reduce] = SnailfishNumber::Number(0);
                    }
                    if let Some(first_right_number) = res.get_first_right_parent_number(will_reduce)
                    {
                        let right_child_number = res.nodes[right_child_number_id];
                        res.nodes[first_right_number] = SnailfishNumber::Number(
                            res.nodes[first_right_number].get_num().unwrap()
                                + right_child_number.get_num().unwrap(),
                        );
                    } else {
                        res.nodes[will_reduce] = SnailfishNumber::Number(0);
                    }
                    res.nodes.remove(left_child_number_id);
                    res.parent.remove(left_child_number_id);
                    res.nodes.remove(right_child_number_id);
                    res.parent.remove(right_child_number_id);
                }
                // check for split
                if let Some(will_split) = (0..res.nodes.len()).find(|node_id| {
                    res.nodes[*node_id].is_number() && res.nodes[*node_id].get_num().unwrap() > 9
                }) {
                    has_made_something = true;
                    println!(
                        "YAOURT will split {} => {:?} (parent: {:?})",
                        will_split, res.nodes[will_split], res.parent[will_split]
                    );
                    let current_number = res.nodes[will_split].get_num().unwrap();
                    let left_value = (current_number - (current_number % 2)) / 2;
                    let right_value =
                        (current_number - (current_number % 2)) / 2 + (current_number % 2);
                    let left_number_id = res.add_number(SnailfishNumber::Number(left_value));
                    res.set_parent(Some(will_split), left_number_id);
                    let right_number_id = res.add_number(SnailfishNumber::Number(right_value));
                    res.set_parent(Some(will_split), right_number_id);
                    let pair = SnailfishNumber::Pair(left_number_id, right_number_id);
                    res.replace(will_split, pair);
                }

                if !has_made_something {
                    break;
                }
                has_made_something = false;
            }
        }
        return res;
    }
}

impl std::str::FromStr for SnailfishTree {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_numeric(it: &mut Peekable<Chars>) -> u32 {
            let mut digits: String = String::new();
            while let Some(cur) = it.peek() {
                if *cur == ',' || *cur == ']' {
                    break;
                }
                digits.push(it.next().unwrap());
            }
            u32::from_str_radix(digits.as_str(), 10).unwrap()
        }
        let mut res = SnailfishTree {
            nodes: Vec::new(),
            parent: Vec::new(),
        };
        let mut stack: Vec<NodeId> = Vec::new();
        let mut it = s.chars().peekable();
        while let Some(current) = it.peek() {
            match current {
                '[' => {
                    it.next();
                }
                ']' => {
                    let right_id = stack.pop().unwrap();
                    let left_id = stack.pop().unwrap();

                    let n = SnailfishNumber::Pair(left_id, right_id);

                    let node_id = res.add_number(n);
                    stack.push(node_id);
                    res.set_parent(Some(node_id), right_id);
                    res.set_parent(Some(node_id), left_id);
                    it.next();
                }
                ',' => {
                    it.next();
                }
                _ => {
                    let n = SnailfishNumber::Number(parse_numeric(&mut it));
                    let node_id = res.add_number(n);
                    stack.push(node_id);
                }
            }
        }

        return Ok(res);
    }
}

type Input = Vec<SnailfishTree>;
type Part1Output = u32;
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
    let lines: Input = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| file_fragment.parse::<SnailfishTree>().unwrap())
        .collect();
    return Ok(lines);
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    let sum = input
        .iter()
        .reduce(|acc, cur| {
            acc.add_and_reduce(cur);
            acc
        })
        .unwrap();
    return Ok(compute_magnitude(sum));
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    return Ok(input.len());
}

fn compute_magnitude(tree: &SnailfishTree) -> u32 {
    fn rec(t: &SnailfishTree, node_id: NodeId) -> u32 {
        let node = t.nodes[node_id];
        match node {
            SnailfishNumber::Number(n) => n,
            SnailfishNumber::Pair(l, r) => 3 * rec(t, l) + 2 * rec(t, r),
        }
    }

    let root_id = tree.get_root_id().unwrap();
    return rec(tree, root_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        parse_data(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]"
                .to_string(),
        )
        .unwrap()
    }

    #[test]
    #[ignore]
    fn it_should_parse_correctly_1() {
        let input = "[1,2]";
        let expected = SnailfishTree {
            nodes: vec![
                SnailfishNumber::Number(1),
                SnailfishNumber::Number(2),
                SnailfishNumber::Pair(0, 1),
            ],
            parent: vec![Some(2), Some(2), None],
        };
        assert_eq!(input.parse::<SnailfishTree>().unwrap(), expected);
    }

    #[test]
    #[ignore]
    fn it_should_parse_correctly_2() {
        let input = "[[1,2],3]";
        let expected = SnailfishTree {
            nodes: vec![
                SnailfishNumber::Number(1),
                SnailfishNumber::Number(2),
                SnailfishNumber::Pair(0, 1),
                SnailfishNumber::Number(3),
                SnailfishNumber::Pair(2, 3),
            ],
            parent: vec![Some(2), Some(2), Some(4), Some(4), None],
        };
        assert_eq!(input.parse::<SnailfishTree>().unwrap(), expected);
    }

    #[test]
    #[ignore]
    fn it_should_compute_magnitude_1() {
        let input = "[[1,2],[[3,4],5]]".parse::<SnailfishTree>().unwrap();
        assert_eq!(compute_magnitude(&input), 143);
    }
    #[test]
    #[ignore]
    fn it_should_compute_magnitude_2() {
        let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
            .parse::<SnailfishTree>()
            .unwrap();
        assert_eq!(compute_magnitude(&input), 1384);
    }
    #[test]
    #[ignore]
    fn it_should_compute_magnitude_3() {
        let input = "[[[[1,1],[2,2]],[3,3]],[4,4]]"
            .parse::<SnailfishTree>()
            .unwrap();
        assert_eq!(compute_magnitude(&input), 445);
    }
    #[test]
    #[ignore]
    fn it_should_compute_magnitude_4() {
        let input = "[[[[3,0],[5,3]],[4,4]],[5,5]]"
            .parse::<SnailfishTree>()
            .unwrap();
        assert_eq!(compute_magnitude(&input), 791);
    }
    #[test]
    #[ignore]
    fn it_should_compute_magnitude_5() {
        let input = "[[[[5,0],[7,4]],[5,5]],[6,6]]"
            .parse::<SnailfishTree>()
            .unwrap();
        assert_eq!(compute_magnitude(&input), 1137);
    }
    #[test]
    #[ignore]
    fn it_should_compute_magnitude_6() {
        let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse::<SnailfishTree>()
            .unwrap();
        assert_eq!(compute_magnitude(&input), 3488);
    }
    #[test]
    #[ignore]
    fn it_should_compute_magnitude_example() {
        let input = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            .parse::<SnailfishTree>()
            .unwrap();
        assert_eq!(compute_magnitude(&input), 4140);
    }

    #[test]
    fn it_should_compute_part1_correctly() {
        let input = sample_input();
        let expected = 4140;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    #[ignore]
    fn it_should_compute_part2_correctly() {
        let input = sample_input();
        let expected = 1;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
