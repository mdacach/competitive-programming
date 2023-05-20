use std::collections::HashMap;

fn get_factors(mut number: i64) -> HashMap<i64, i64> {
    if number == 1 {
        return HashMap::new();
    }

    let mut factors = HashMap::new();

    let mut possible_divisor = 2;
    while possible_divisor * possible_divisor <= number {
        while number % possible_divisor == 0 {
            *factors.entry(possible_divisor).or_default() += 1;
            number /= possible_divisor;
        }

        possible_divisor += 1;
    }

    if number > 1 {
        *factors.entry(number).or_default() += 1;
    }

    factors
}

fn main() {
    let mut scanner = Scanner::new();

    let t = scanner.next();
    'testcase: for _ in 0..t {
        let a: i64 = scanner.next();
        let b: i64 = scanner.next();
        let c: i64 = scanner.next();
        let d: i64 = scanner.next();

        if a * b == 1 {
            println!("{c} {d}");
            continue 'testcase;
        }

        let wanted_factors = get_factors(a * b);
        for possible_x in (a + 1)..=c {
            let factors = get_factors(possible_x);

            let mut missing = 1;
            for (prime, &wanted_exponent) in &wanted_factors {
                let exponent_we_have = if let Some(e) = factors.get(prime) {
                    *e
                } else {
                    0
                };

                if exponent_we_have < wanted_exponent {
                    for _ in 0..(wanted_exponent - exponent_we_have) {
                        missing *= prime;
                    }
                }
            }

            // Check if there is a multiple of `missing` in range (b, d]
            let mut right = (d / missing) + 1; // Always too big.
            let mut left = 0;
            let mut answer = None;
            while right - left > 1 {
                let mid = left + (right - left) / 2;
                let current_value = mid * missing;
                if current_value <= b {
                    left = mid;
                } else if current_value > d {
                    right = mid;
                } else {
                    answer = Some(current_value);
                    break;
                }
            }

            if let Some(answer) = answer {
                println!("{possible_x} {answer}");
                continue 'testcase;
            }
        }
        println!("-1 -1");
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
