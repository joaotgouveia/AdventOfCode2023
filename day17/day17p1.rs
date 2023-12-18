use std::fs;

fn extract_graph(map: Vec<Vec<u32>>) -> Vec<Vec<(usize, u32)>> {
    let mut graph: Vec<Vec<(usize, u32)>> = Vec::new();
    let line_count = map.len();
    let line_length = map[0].len();

    for i in 0..line_count * line_length {
        graph.push(Vec::new());
    }

    for i in 0..line_count {
        for j in 0..line_length {
            let node_num = i * line_length + j;
            if i > 0 {
                graph[node_num].push(((i - 1) * line_length + j, map[i - 1][j]));
            }
            if i < line_count - 1 {
                graph[node_num].push(((i + 1) * line_length + j, map[i + 1][j]));
            }
            if j > 0 {
                graph[node_num].push((i * line_length + j - 1, map[i][j - 1]));
            }
            if j < line_length - 1 {
                graph[node_num].push((i * line_length + j + 1, map[i][j + 1]));
            }
        }
    }

    graph
}

fn least_heat_loss(graph: Vec<Vec<(usize, u32)>>) -> u32 {
    let mut direction_count = 0;
    0
}

fn main() {
    let document = fs::read_to_string("day13.in").unwrap();
    let map = document
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let graph = extract_graph(map);
    let heat_loss = least_heat_loss(graph);

    println!("{}", heat_loss);
}
