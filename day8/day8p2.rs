use std::cmp;
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

fn count_steps(starting_node: &Node, nodes: &HashMap<String, Node>, instructions: &str) -> u64 {
    let mut steps = 0;
    let mut current_node = starting_node;
    loop {
        for dir in instructions.chars() {
            steps += 1;
            match dir {
                'L' => current_node = nodes.get(&current_node.left).unwrap(),
                'R' => current_node = nodes.get(&current_node.right).unwrap(),
                _ => panic!("Unrecognized instruction"),
            }
            if current_node.label.chars().nth(2).unwrap() == 'Z' {
                return steps;
            }
        }
    }
}

fn gcm(a: u64, b: u64) -> u64 {
    let max = cmp::max(a, b);
    let min = cmp::min(a, b);

    if min == 0 {
        return max;
    }

    gcm(max % min, min)
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcm(a, b)
}

fn vec_lcm(initial_nums: Vec<u64>) -> u64 {
    let mut nums = initial_nums;
    while nums.len() != 1 {
        let mut new_nums: Vec<u64> = Vec::new();
        let size = nums.len();
        for i in (0..size - 1).step_by(2) {
            new_nums.push(lcm(nums[i], nums[i + 1]));
        }
        if size % 2 != 0 {
            new_nums.push(nums[size - 1]);
        }
        nums = new_nums;
    }
    nums[0]
}

fn main() {
    let document = fs::read_to_string("day8.in").unwrap();
    let mut lines = document.lines();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut steps: Vec<u64> = Vec::new();
    let mut starting_nodes: Vec<Node> = Vec::new();
    let instructions = lines.next().unwrap();

    lines.next();
    for line in lines {
        let node = Node::parse(line);
        if node.label.chars().nth(2).unwrap() == 'A' {
            starting_nodes.push(node);
        } else {
            nodes.insert(node.label.clone(), node);
        }
    }

    for node in starting_nodes {
        steps.push(count_steps(&node, &nodes, instructions));
    }

    println!("{}", vec_lcm(steps));
}
