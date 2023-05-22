use std::collections::HashMap;

fn get_factors(mut num: i64) -> HashMap<i64, i64> {
    let mut factors = HashMap::new();

    let mut possible_divisor = 2;
    while possible_divisor * possible_divisor <= num {
        while num % possible_divisor == 0 {
            *factors.entry(possible_divisor).or_default() += 1;
            num /= possible_divisor;
        }
        possible_divisor += 1;
    }

    if num > 1 {
        *factors.entry(num).or_default() += 1;
    }

    factors
}

fn get_divisors(num: i64) -> Vec<i64> {
    let factors = get_factors(num);

    let mut divisors = vec![1];

    for (prime, exponent) in factors {
        let mut new_divisors = Vec::new();
        for d in &divisors {
            for chosen in 1..=exponent {
                new_divisors.push(d * prime.pow(chosen as u32));
            }
        }

        divisors.extend(new_divisors);
    }

    divisors
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
            println!("{} {}", c, d);
            continue 'testcase;
        }

        for d1 in get_divisors(a) {
            for d2 in get_divisors(b) {
                let possible_x = d1 * d2;

                let possible_x = (c / possible_x) * possible_x;
                if possible_x <= a || possible_x > c {
                    continue;
                }

                let wanted_from_y = a * b / gcd(a * b, possible_x);

                let possible_y = (d / wanted_from_y) * wanted_from_y;
                if possible_y > b {
                    println!("{} {}", possible_x, possible_y);
                    continue 'testcase;
                }
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
