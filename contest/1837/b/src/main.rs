use std::cmp::max;
use std::collections::HashSet;

fn main() {
    let mut scanner = Scanner::new();

    let t = scanner.next();
    'testcase: for _ in 0..t {
        let n: usize = scanner.next();
        let s: String = scanner.next();

        let chars: Vec<char> = s.chars().collect();

        let mut maximum_consecutive = 1;

        let mut current_char = chars[0];
        let mut current_consecutive = 1;
        for c in chars.iter().skip(1) {
            if *c == current_char {
                current_consecutive += 1;
            } else {
                current_char = *c;
                current_consecutive = 1;
            }
            maximum_consecutive = max(maximum_consecutive, current_consecutive);
        }
        maximum_consecutive = max(maximum_consecutive, current_consecutive);

        println!("{}", maximum_consecutive + 1);
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
