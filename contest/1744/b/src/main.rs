fn main() {
    let mut s = Scanner::new();

    let t = s.next();
    'testcase: for _ in 0..t {
        let n = s.next();
        let q = s.next();
        let elements = (0..n).map(|_| s.next()).collect::<Vec<i64>>();

        let mut current_sum: i64 = elements.iter().sum();
        let mut count_even = elements.iter().filter(|&&x| x % 2 == 0).count() as i64;
        let mut count_odd = elements.iter().filter(|&&x| x % 2 == 1).count() as i64;

        for _ in 0..q {
            let operation_type = s.next();
            let value_to_add: i64 = s.next();
            let value_to_add_is_even = value_to_add % 2 == 0;

            match operation_type {
                0 => {
                    current_sum += count_even * value_to_add;
                    if value_to_add_is_even {
                    } else {
                        count_odd += count_even;
                        count_even = 0;
                    }
                }
                1 => {
                    current_sum += count_odd * value_to_add;
                    if value_to_add_is_even {
                    } else {
                        count_even += count_odd;
                        count_odd = 0;
                    }
                }
                _ => panic!("invalid input."),
            }

            println!("{}", current_sum);
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
