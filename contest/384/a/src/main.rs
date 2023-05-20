fn main() {
    let mut s = Scanner::new();

    let size = s.next();
    let count = if size % 2 == 0 {
        size * size / 2
    } else {
        size * size / 2 + 1
    };

    println!("{}", count);
    for i in 0..size {
        for j in 0..size {
            if (i + j) % 2 == 0 {
                print!("C");
            } else {
                print!(".");
            }
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
