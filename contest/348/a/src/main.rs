fn main() {
    let mut scanner = Scanner::new();

    let n = scanner.next();
    let values = (0..n).map(|_| scanner.next()).collect::<Vec<i64>>();
    let total: i64 = values.iter().sum();
    let max = *values.iter().max().unwrap();

    println!("{}", std::cmp::max((total + n - 2) / (n - 1), max));
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
