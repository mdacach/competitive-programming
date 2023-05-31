fn main() {
    let mut scanner = Scanner::new();
    let t = scanner.next();
    'testcase: for _ in 0..t {
        let n = scanner.next();
        let s: String = scanner.next();

        let s: Vec<char> = s.chars().collect();
        'is_possible: {
            let mut count = 0;
            for &c in &s {
                if c == '(' {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            if count != 0 {
                println!("-1");
                continue 'testcase;
            }
        }

        'only_needs_one: {
            let mut count = 0;
            let mut good = true;
            for &c in &s {
                if c == '(' {
                    count += 1;
                } else {
                    count -= 1;
                }

                if count < 0 {
                    good = false;
                }
            }

            if good {
                println!("1");
                println!("{}", "1 ".repeat(n));
                continue 'testcase;
            }

            let mut count = 0;
            let mut good = true;
            for &c in s.iter().rev() {
                if c == '(' {
                    count += 1;
                } else {
                    count -= 1;
                }

                if count < 0 {
                    good = false;
                }
            }

            if good {
                println!("1");
                println!("{}", "1 ".repeat(n));
                continue 'testcase;
            }
        }

        let mut count = 0;
        let mut bad = 0;
        let mut colors = vec![0; n];
        for i in 0..n {
            if s[i] == '(' {
                colors[i] = 1;
                count += 1;
            } else {
                if count == 0 {
                    colors[i] = 2;
                    bad += 1;
                } else {
                    colors[i] = 1;
                    count -= 1;
                }
            }
        }

        for i in (0..n).rev() {
            if s[i] == '(' && colors[i] == 1 {
                if bad >= 1 {
                    bad -= 1;
                    colors[i] = 2;
                }
            }
        }

        println!("2");
        for x in colors {
            print!("{x} ");
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
