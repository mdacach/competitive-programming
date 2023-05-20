struct Solution {
    matrix: Vec<Vec<i32>>,
    visited: Vec<Vec<i32>>,
}

impl Solution {
    fn new() -> Self {
        Self {
            matrix: vec![],
            visited: vec![],
        }
    }

    fn visit(&mut self, i: i32, j: i32) -> Result<(), ()> {
        let size = self.matrix.len() as i32;
        if i < 0 || i >= size {
            return Err(());
        }
        if j < 0 || j >= size {
            return Err(());
        }
        if self.matrix[i as usize][j as usize] == 0 {
            return Err(());
        }
        self.visited[i as usize][j as usize] += 1;

        Ok(())
    }

    fn solve(&mut self) {
        let mut s = Scanner::new();

        let size = s.next();
        let mut matrix = vec![vec![0; size]; size];
        for i in 0..size {
            let line: String = s.next();
            let line: Vec<_> = line.chars().collect();
            for j in 0..size {
                if line[j] == '#' {
                    matrix[i][j] = 1;
                } else {
                    matrix[i][j] = 0;
                }
            }
        }
        self.matrix = matrix;
        let visited = vec![vec![0; size]; size];
        self.visited = visited;

        for i in 0..size {
            for j in 0..size {
                if self.matrix[i][j] == 0 || self.visited[i][j] != 0 {
                    continue;
                }
                let (i, j) = (i as i32, j as i32);
                if self.visit(i, j).is_err() {
                    println!("NO");
                    return;
                }
                if self.visit(i + 1, j - 1).is_err() {
                    println!("NO");
                    return;
                }
                if self.visit(i + 1, j).is_err() {
                    println!("NO");
                    return;
                }
                if self.visit(i + 1, j + 1).is_err() {
                    println!("NO");
                    return;
                }
                if self.visit(i + 2, j).is_err() {
                    println!("NO");
                    return;
                }
            }
        }

        let visited_multiple_times = self.visited.iter().flatten().any(|x| *x > 1);
        if visited_multiple_times {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut s = Solution::new();
    s.solve();
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
