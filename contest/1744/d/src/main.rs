fn main() {
    let mut scanner = Scanner::new();

    let t = scanner.next();
    'testcase: for _ in 0..t {
        let n = scanner.next();
        let values: Vec<i32> = (0..n).map(|_| scanner.next()).collect();

        let mut factors_we_have = 0;
        for mut v in values {
            let mut this_factors = 0;
            while v % 2 == 0 {
                this_factors += 1;
                v /= 2;
            }
            factors_we_have += this_factors;
        }

        let factors_we_need = n;

        if factors_we_have >= factors_we_need {
            println!("0");
            continue 'testcase;
        } else {
            let mut answer = 0;

            let mut missing_factors = factors_we_need - factors_we_have;
            let mut factors_we_can_add: Vec<i32> = (1..=n)
                .map(|mut v| {
                    let mut this_factors = 0;
                    while v % 2 == 0 {
                        this_factors += 1;
                        v /= 2;
                    }
                    this_factors
                })
                .collect();
            factors_we_can_add.sort_unstable_by(|a, b| b.cmp(a));

            for f in factors_we_can_add {
                missing_factors -= f;
                answer += 1;

                if missing_factors <= 0 {
                    println!("{answer}");
                    continue 'testcase;
                }
            }

            println!("-1");
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
