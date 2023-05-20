use std::collections::HashMap;

fn main() {
    let mut s = Scanner::new();

    let t = s.next();
    'testcase: for _ in 0..t {
        let n = s.next();
        let values = (0..n).map(|_| s.next()).collect::<Vec<u32>>();
        let string: String = s.next();
        let string = string.chars().collect::<Vec<char>>();

        let mut map: HashMap<u32, Vec<usize>> = HashMap::new();
        for (i, v) in values.iter().enumerate() {
            map.entry(*v).or_default().push(i);
        }

        for (k, v) in map {
            let mut chars = v.iter().map(|&i| string[i]).collect::<Vec<_>>();
            chars.sort();
            if chars.first().unwrap() != chars.last().unwrap() {
                println!("NO");
                continue 'testcase;
            }
        }

        println!("YES");
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
