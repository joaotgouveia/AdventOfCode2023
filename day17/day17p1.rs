use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

struct Vertex {
    v: usize,
    d: u32,
}

impl Eq for Vertex {}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.d.partial_cmp(&self.d)
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_graph(weights: &Vec<Vec<u32>>) -> (Vec<Vec<usize>>, BinaryHeap<Vertex>) {
    let mut adj: Vec<Vec<usize>> = Vec::new();
    let mut vertexes: BinaryHeap<Vertex> = BinaryHeap::new();
    let line_count = weights.len();
    let line_length = weights[0].len();

    adj.push(Vec::new());
    vertexes.push(Vertex { v: 0, d: 0 });
    for i in 1..line_count * line_length {
        adj.push(Vec::new());
        vertexes.push(Vertex { v: i, d: u32::MAX });
    }

    for i in 0..line_count {
        for j in 0..line_length {
            let node_num = i * line_length + j;
            if i > 0 {
                adj[node_num].push((i - 1) * line_length + j);
            }
            if i < line_count - 1 {
                adj[node_num].push((i + 1) * line_length + j);
            }
            if j > 0 {
                adj[node_num].push(i * line_length + j - 1);
            }
            if j < line_length - 1 {
                adj[node_num].push(i * line_length + j + 1);
            }
        }
    }

    (adj, vertexes)
}

fn w(weights: &Vec<Vec<u32>>, v: usize) -> u32 {
    let line_length = weights[0].len();
    let i = v / line_length + 1;
    let j = v % line_length;

    weights[i][j]
}

fn least_heat_loss(adj: Vec<Vec<usize>>, weights: Vec<Vec<u32>>) -> u32 {
    let mut direction_count = 0;
    0
}

fn main() {
    let document = fs::read_to_string("day13.in").unwrap();
    let weights = document
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let (adj, vertexes) = get_graph(&weights);
    let heat_loss = least_heat_loss(adj, weights);

    println!("{}", heat_loss);
}
