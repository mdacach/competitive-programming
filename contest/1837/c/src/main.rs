fn main() {
    let mut scanner = Scanner::new();
    let t = scanner.next();
    'testcase: for _ in 0..t {
        let s: String = scanner.next();

        match s.chars().position(|x| x != '?') {
            None => {
                println!("{}", "0".repeat(s.len()));
            }
            Some(p) => {
                let mut as_chars: Vec<char> = s.chars().collect();

                let mut previous = as_chars[p];
                for c in &mut as_chars {
                    if *c == '?' {
                        *c = previous;
                    } else {
                        previous = *c;
                    }
                }

                println!("{}", as_chars.iter().collect::<String>());
            }
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
