use std::cmp::{min, Ordering};

fn main() {
    let mut s = Scanner::new();

    let n: usize = s.next();
    let digits = (0..n).map(|_| s.next()).collect::<Vec<u8>>();

    if !digits.iter().any(|&x| x == 0) {
        println!("-1");
        return;
    }

    let zeros = digits.iter().filter(|&&d| d == 0);
    let non_zeros = digits.iter().filter(|&&d| d != 0);

    let mut rest_zero = non_zeros
        .clone()
        .filter(|&&d| d % 3 == 0)
        .collect::<Vec<_>>();

    let mut rest_one = non_zeros
        .clone()
        .filter(|&&d| d % 3 == 1)
        .collect::<Vec<_>>();
    let mut rest_one_count = rest_one.len();

    let mut rest_two = non_zeros
        .clone()
        .filter(|&&d| d % 3 == 2)
        .collect::<Vec<_>>();
    let mut rest_two_count = rest_two.len();

    let mut pick_one = 0;
    let mut pick_two = 0;

    match rest_one_count.cmp(&rest_two_count) {
        Ordering::Less => {
            let more = rest_two_count - rest_one_count;
            let to_pick = if (more + 1) % 3 == 0 && more < rest_two_count {
                more + 1
            } else {
                more
            };
            pick_two += to_pick / 3 * 3;
            rest_two_count -= to_pick / 3 * 3;
        }
        Ordering::Equal => {}
        Ordering::Greater => {
            let more = rest_one_count - rest_two_count;
            let to_pick = if (more + 1) % 3 == 0 && more < rest_one_count {
                more + 1
            } else {
                more
            };
            pick_one += to_pick / 3 * 3;
            rest_one_count -= to_pick / 3 * 3;
        }
    }

    pick_one += min(rest_one_count, rest_two_count);
    pick_two += min(rest_one_count, rest_two_count);

    rest_zero.sort();
    rest_one.sort();
    rest_two.sort();

    let mut pick_zero = rest_zero.len();

    let mut answer = vec![];
    while pick_zero > 0 || pick_one > 0 || pick_two > 0 {
        let mut candidates = vec![];
        if pick_zero > 0 {
            candidates.push(*rest_zero.last().unwrap());
        }
        if pick_one > 0 {
            candidates.push(*rest_one.last().unwrap());
        }
        if pick_two > 0 {
            candidates.push(*rest_two.last().unwrap());
        }

        let chosen = *candidates.into_iter().max().unwrap();
        answer.push(chosen);

        match chosen % 3 {
            0 => {
                pick_zero -= 1;
                rest_zero.pop();
            }
            1 => {
                pick_one -= 1;
                rest_one.pop();
            }
            2 => {
                pick_two -= 1;
                rest_two.pop();
            }
            _ => panic!(),
        }
    }

    (0..(zeros.count())).for_each(|_| answer.push(0));

    if *answer.first().unwrap() == 0 {
        println!("0");
        return;
    }

    for d in answer {
        print!("{d}");
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
