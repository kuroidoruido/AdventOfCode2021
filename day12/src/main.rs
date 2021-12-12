use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq, Debug)]
enum Node {
    Small(String, Vec<String>),
    Big(String, Vec<String>),
}

#[derive(Clone, PartialEq, Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
    start: Node,
}

type Input = Graph;
type Path = Vec<String>;
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
    let connections: Vec<(String, String)> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| file_fragment.split_once("-"))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(|(a, b)| (String::from(a), String::from(b)))
        .collect();

    let mut nodes_map: HashMap<String, Vec<String>> = HashMap::new();
    for (from, to) in connections.iter() {
        if *from != String::from("end") && *to != String::from("start") {
            if let Some(previous_to_list) = nodes_map.get_mut(from) {
                previous_to_list.push(to.clone());
            } else {
                let to_list: Vec<String> = vec![to.clone()];
                nodes_map.insert(from.clone(), to_list);
            }
        }
        if *to != String::from("end") && *from != String::from("start") {
            if let Some(previous_to_list) = nodes_map.get_mut(to) {
                previous_to_list.push(from.clone());
            } else {
                let to_list: Vec<String> = vec![from.clone()];
                nodes_map.insert(to.clone(), to_list);
            }
        }
    }

    let nodes: Vec<Node> = nodes_map
        .iter()
        .map(|(from, to)| {
            if is_big(from) {
                Node::Big(from.clone(), to.clone())
            } else {
                Node::Small(from.clone(), to.clone())
            }
        })
        .collect();

    let nodes: HashMap<String, Node> = nodes
        .iter()
        .map(|node| {
            let name = match node {
                Node::Small(node_name, _) => node_name,
                Node::Big(node_name, _) => node_name,
            };
            return (name.clone(), node.clone());
        })
        .collect();

    let start = nodes
        .get(&String::from("start"))
        .expect("should have a start node")
        .clone();

    return Ok(Graph { nodes, start });
}

fn is_big(s: &String) -> bool {
    s.to_ascii_uppercase() == *s
}

fn part1(input: &Input) -> Result<Part1Output, String> {
    let all_path = get_all_path_1(input);
    return Ok(all_path.len());
}

fn part2(input: &Input) -> Result<Part2Output, String> {
    let all_path = get_all_path_2(input);
    return Ok(all_path.len());
}

fn get_all_path_1(graph: &Graph) -> Vec<Path> {
    fn explore(graph: &Graph, path: &Path, node_name: String) -> Vec<Path> {
        if node_name == String::from("end") {
            return vec![vec![path.clone(), vec![String::from("end")]].concat()];
        }

        return match graph.nodes.get(&node_name) {
            Some(Node::Big(_, next_nodes)) => next_nodes
                .iter()
                .map(|n| {
                    explore(
                        graph,
                        &vec![path.clone(), vec![node_name.clone()]].concat(),
                        n.clone(),
                    )
                })
                .fold(vec![], |acc, cur| [acc, cur].concat()),
            Some(Node::Small(_, next_nodes)) => {
                if path.contains(&node_name) {
                    vec![]
                } else {
                    next_nodes
                        .iter()
                        .map(|n| {
                            explore(
                                graph,
                                &vec![path.clone(), vec![node_name.clone()]].concat(),
                                n.clone(),
                            )
                        })
                        .fold(vec![], |acc, cur| [acc, cur].concat())
                }
            }
            None => vec![],
        };
    }
    return explore(graph, &vec![], String::from("start"));
}

fn get_all_path_2(graph: &Graph) -> Vec<Path> {
    fn explore(
        graph: &Graph,
        path: &Path,
        node_name: String,
        small_twice: Option<String>,
    ) -> Vec<Path> {
        if node_name == String::from("end") {
            return vec![vec![path.clone(), vec![String::from("end")]].concat()];
        }

        return match graph.nodes.get(&node_name) {
            Some(Node::Big(_, next_nodes)) => next_nodes
                .iter()
                .map(|n| {
                    explore(
                        graph,
                        &vec![path.clone(), vec![node_name.clone()]].concat(),
                        n.clone(),
                        small_twice.clone(),
                    )
                })
                .fold(vec![], |acc, cur| [acc, cur].concat()),
            Some(Node::Small(_, next_nodes)) => {
                if path.contains(&node_name) {
                    if small_twice.is_some() {
                        vec![]
                    } else {
                        next_nodes
                            .iter()
                            .map(|n| {
                                explore(
                                    graph,
                                    &vec![path.clone(), vec![node_name.clone()]].concat(),
                                    n.clone(),
                                    Some(node_name.clone()),
                                )
                            })
                            .fold(vec![], |acc, cur| [acc, cur].concat())
                    }
                } else {
                    next_nodes
                        .iter()
                        .map(|n| {
                            explore(
                                graph,
                                &vec![path.clone(), vec![node_name.clone()]].concat(),
                                n.clone(),
                                small_twice.clone(),
                            )
                        })
                        .fold(vec![], |acc, cur| [acc, cur].concat())
                }
            }
            None => vec![],
        };
    }
    return explore(graph, &vec![], String::from("start"), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        let mut graph = Graph {
            nodes: HashMap::new(),
            start: Node::Small(
                String::from("start"),
                vec![String::from("A"), String::from("b")],
            ),
        };
        graph.nodes.insert(
            String::from("start"),
            Node::Small(
                String::from("start"),
                vec![String::from("A"), String::from("b")],
            ),
        );
        graph.nodes.insert(
            String::from("A"),
            Node::Big(
                String::from("A"),
                vec![String::from("c"), String::from("b"), String::from("end")],
            ),
        );
        graph.nodes.insert(
            String::from("b"),
            Node::Small(
                String::from("b"),
                vec![String::from("A"), String::from("d"), String::from("end")],
            ),
        );
        graph.nodes.insert(
            String::from("c"),
            Node::Small(String::from("c"), vec![String::from("A")]),
        );
        graph.nodes.insert(
            String::from("d"),
            Node::Small(String::from("d"), vec![String::from("b")]),
        );
        return graph;
    }

    #[test]
    fn it_should_parse_correctly() {
        let input = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";
        let expected = sample_input();
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly_small() {
        let input = sample_input();
        let expected = 10;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly_middle() {
        let input = parse_data(String::from(
            "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc",
        ))
        .unwrap();
        let expected = 19;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part1_correctly_larger() {
        let input = parse_data(String::from(
            "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
        ",
        ))
        .unwrap();
        let expected = 226;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly_small() {
        let input = sample_input();
        let expected = 36;
        assert_eq!(part2(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly_middle() {
        let input = parse_data(String::from(
            "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc",
        ))
        .unwrap();
        let expected = 103;
        assert_eq!(part2(&input).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_part2_correctly_larger() {
        let input = parse_data(String::from(
            "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
        ",
        ))
        .unwrap();
        let expected = 3509;
        assert_eq!(part2(&input).unwrap(), expected);
    }
}
