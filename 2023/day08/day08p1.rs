use std::collections::HashMap;
use std::fs;

struct Node {
    label: String,
    left: String,
    right: String,
}

impl Node {
    fn parse(line: &str) -> Self {
        let nodes: Vec<&str> = line
            .split(|x: char| !x.is_alphabetic())
            .filter(|x| !x.is_empty())
            .collect();
        Self {
            label: String::from(nodes[0]),
            left: String::from(nodes[1]),
            right: String::from(nodes[2]),
        }
    }
}

fn count_steps(nodes: HashMap<String, Node>, instructions: &str) -> u32 {
    let mut steps = 0;
    let mut current_node = nodes.get("AAA").unwrap();
    loop {
        for dir in instructions.chars() {
            steps += 1;
            match dir {
                'L' => current_node = nodes.get(&current_node.left).unwrap(),
                'R' => current_node = nodes.get(&current_node.right).unwrap(),
                _ => panic!("Unrecognized instruction"),
            }
            if current_node.label.eq("ZZZ") {
                return steps;
            }
        }
    }
}

fn main() {
    let document = fs::read_to_string("day8.in").unwrap();
    let mut lines = document.lines();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let instructions = lines.next().unwrap();
    lines.next();
    for line in lines {
        let node = Node::parse(line);
        nodes.insert(node.label.clone(), node);
    }

    println!("{}", count_steps(nodes, instructions));
}
