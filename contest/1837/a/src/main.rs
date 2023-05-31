fn main() {
    let mut scanner = Scanner::new();
    let t = scanner.next();
    'testcase: for _ in 0..t {
        let (x, k): (i32, i32) = (scanner.next(), scanner.next());
        if x % k != 0 {
            println!("1");
            println!("{x}");
        } else {
            println!("2");
            println!("1 {}", x - 1);
        }
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
