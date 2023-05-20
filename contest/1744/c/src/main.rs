use std::cmp::max;

fn main() {
    let mut scanner = Scanner::new();
    let t = scanner.next();
    'testcase: for _ in 0..t {
        let n: usize = scanner.next();
        let c: char = scanner.next();
        let mut s = {
            let s: String = scanner.next();
            s.chars().collect::<Vec<_>>()
        };
        s.extend(s.clone());

        let mut answer_r = 0;
        let mut answer_y = 0;
        let mut last_green = None;
        for i in (0..(2 * n)).rev() {
            let current_char = s[i];
            match current_char {
                'g' => {
                    last_green = Some(i);
                }
                other => {
                    if let Some(pos) = last_green {
                        if other == 'r' {
                            answer_r = max(answer_r, pos - i);
                        } else {
                            assert_eq!(other, 'y');
                            answer_y = max(answer_y, pos - i);
                        }
                    };
                }
            }
        }

        match c {
            'r' => println!("{}", answer_r),
            'y' => println!("{}", answer_y),
            'g' => println!("{}", 0),
            _ => panic!("invalid input."),
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
