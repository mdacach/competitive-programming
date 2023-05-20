use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};
use std::str::FromStr;

struct Scanner {
    tokens: VecDeque<String>,
}

impl Scanner {
    pub fn new() -> Self {
        let stdin = io::stdin();
        let mut tokens = VecDeque::new();
        for line in stdin.lock().lines() {
            for token in line.unwrap().split_ascii_whitespace() {
                tokens.push_back(token.to_owned());
            }
        }
        Self { tokens }
    }

    pub fn next<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.tokens.pop_front().unwrap().parse().unwrap()
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.next();
    let m: usize = sc.next();
    let d: i32 = sc.next();

    let mut p = HashSet::new();
    for _ in 0..m {
        let idx: usize = sc.next();
        p.insert(idx);
    }

    let mut g = vec![Vec::new(); n + 1];
    for _ in 0..(n - 1) {
        let x: usize = sc.next();
        let y: usize = sc.next();
        g[x].push(y);
        g[y].push(x);
    }

    let mut dist = vec![vec![i32::MIN; 3]; n + 1];
    fn dfs(
        x: usize,
        fa: i32,
        g: &Vec<Vec<usize>>,
        dist: &mut Vec<Vec<i32>>,
        p: &HashSet<usize>,
    ) -> i32 {
        let (mut first, mut second, mut node) = (i32::MIN, i32::MIN, i32::MIN);
        for &y in &g[x] {
            if y == fa as usize {
                continue;
            }
            let sub = 1 + dfs(y, x as i32, g, dist, p);
            if sub > first {
                second = first;
                first = sub;
                node = y as i32;
            } else if sub > second {
                second = sub;
            }
        }
        dist[x][0] = first;
        dist[x][1] = second;
        dist[x][2] = node;
        if first < 0 && p.contains(&x) {
            return 0;
        }
        first
    }
    dfs(1, 0, &g, &mut dist, &p);

    let mut res = 0;
    fn dfs2(
        x: usize,
        fa: i32,
        g: &Vec<Vec<usize>>,
        dist: &Vec<Vec<i32>>,
        p: &HashSet<usize>,
        top: i32,
        res: &mut i32,
        d: i32,
    ) {
        if top > d {
            return;
        }
        let (first, second, node) = (dist[x][0], dist[x][1], dist[x][2]);
        if first <= d {
            *res += 1;
        }
        let mut top = top;
        if p.contains(&x) && top < 0 {
            top = 0;
        }
        for &y in &g[x] {
            if y == fa as usize {
                continue;
            }
            if y == node as usize {
                dfs2(
                    y,
                    x as i32,
                    g,
                    dist,
                    p,
                    std::cmp::max(top, second) + 1,
                    res,
                    d,
                );
            } else {
                dfs2(
                    y,
                    x as i32,
                    g,
                    dist,
                    p,
                    std::cmp::max(top, first) + 1,
                    res,
                    d,
                );
            }
        }
    }
    dfs2(1, 0, &g, &dist, &p, i32::MIN, &mut res, d);
    println!("{}", res);
}
