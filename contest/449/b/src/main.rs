use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem::swap;

fn main() {
    let mut s = Scanner::new();

    let n: usize = s.next();
    let m = s.next();
    let k = s.next();

    let mut adj = vec![vec![]; n + 1];

    (0..m).for_each(|_| {
        let u: usize = s.next();
        let v: usize = s.next();
        let x: i64 = s.next();

        adj[u].push((v, x));
        adj[v].push((u, x));
    });

    let trains = (0..k)
        .map(|_| {
            let x: usize = s.next();
            let c: i64 = s.next();

            (x, c)
        })
        .collect::<Vec<_>>();

    for &(x, c) in &trains {
        adj[1].push((x, c));
    }

    // Dijkstra
    const INF: i64 = i64::MAX;
    let mut min_dist = vec![INF; n + 1];
    let mut processed = vec![false; n + 1];
    let mut count = vec![0; n + 1];
    count[1] = 1;
    // (dist, target)
    min_dist[1] = 0;
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((Reverse(0), 1));
    while !priority_queue.is_empty() {
        let (Reverse(dist), current) = priority_queue.pop().unwrap();
        if processed[current] {
            continue;
        }
        processed[current] = true;

        for &(neighbor, weight) in &adj[current] {
            // relax
            if dist + weight < min_dist[neighbor] {
                count[neighbor] = count[current];
                min_dist[neighbor] = dist + weight;
                priority_queue.push((Reverse(min_dist[neighbor]), neighbor));
            } else if dist + weight == min_dist[neighbor] {
                count[neighbor] += count[current];
            }
        }
    }
    count[1] = 0;

    let mut count_roberio = vec![0; n + 1];
    for i in 1..=n {
        for &(node, weight) in &adj[i] {
            if min_dist[i] == INF {
                continue;
            }
            if min_dist[i] + weight == min_dist[node] {
                count_roberio[node] += 1;
            }
        }
    }

    assert_eq!(count_roberio[1], 0);
    assert_eq!(count, count_roberio);

    swap(&mut count, &mut count_roberio);

    let mut answer = 0;
    // remove the bad ones
    for (node, cost) in trains {
        assert!(cost >= min_dist[node]);
        if cost > min_dist[node] {
            answer += 1;
        } else if count[node] > 1 {
            answer += 1;
            count[node] -= 1;
        }
    }

    println!("{}", answer);
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
