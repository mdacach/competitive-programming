use std::collections::HashMap;

fn main() {
    let mut scanner = Scanner::new();

    let k: u32 = scanner.next();

    const NODES: usize = 1000;
    let add_edge = |a: usize, b: usize, adj: &mut Vec<Vec<bool>>| {
        adj[a][b] = true;
        adj[b][a] = true;
    };

    const START_NODE: usize = 0;
    const END_NODE: usize = 1;
    const MAX_BITS: u32 = 31;

    let mut repr = vec![0; (MAX_BITS + 1) as usize];
    let mut adj = vec![vec![false; NODES]; NODES];
    let mut construct_graph = |mut shortest_paths: u32, current_vertex: &mut usize| {
        let mut current_power = MAX_BITS;
        while shortest_paths % 2 == 0 {
            add_edge(*current_vertex, *current_vertex + 1, &mut adj);

            add_edge(*current_vertex, *current_vertex + 2, &mut adj);
            add_edge(*current_vertex + 1, *current_vertex + 3, &mut adj);
            add_edge(*current_vertex + 2, *current_vertex + 3, &mut adj);

            repr[current_power as usize] = *current_vertex;
            current_power -= 1;

            *current_vertex += 3;
            shortest_paths /= 2;
        }

        repr[current_power as usize] = *current_vertex;

        add_edge(*current_vertex, END_NODE, &mut adj);
        *current_vertex += 1;
    };

    let mut current_vertex = 2;
    construct_graph(1 << MAX_BITS, &mut current_vertex);

    // extra edges
    for i in 0..62 {
        add_edge(current_vertex, current_vertex + 1, &mut adj);
        current_vertex += 1;
    }

    let mut current_distance = 0;
    add_edge(START_NODE, current_vertex, &mut adj);
    for b in (0..=MAX_BITS).rev() {
        if (k >> b) & 1 == 1 {
            let missing_length = 2 * (MAX_BITS - b);
            let delta = missing_length - current_distance;
            for _ in 0..delta {
                add_edge(current_vertex, current_vertex + 1, &mut adj);
                current_vertex += 1;
            }
            current_distance += delta;

            let to_join = repr[b as usize];
            add_edge(current_vertex, to_join, &mut adj);
        }
    }

    println!("{NODES}");
    for row in adj {
        for v in row {
            print!("{}", if v { 'Y' } else { 'N' });
        }
        println!();
    }
}

struct Scanner {
    tokens: std::collections::VecDeque<String>,
}

impl Scanner {
    pub fn new() -> Self {
        use std::io::BufRead;

        let stdin = std::io::stdin();
        let mut tokens = std::collections::VecDeque::new();
        for line in stdin.lock().lines() {
            for token in line.unwrap().split_ascii_whitespace() {
                tokens.push_back(token.to_owned());
            }
        }
        Self { tokens }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        T::from_str(&self.tokens.pop_front().unwrap()).unwrap()
    }
}
