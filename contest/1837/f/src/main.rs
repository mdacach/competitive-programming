use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;
use std::thread::current;

fn main() {
    let mut s = Scanner::new();
    let t = s.next();
    'testcase: for _ in 0..t {
        let (n, k): (usize, usize) = (s.next(), s.next());
        let values: Vec<u64> = (0..n).map(|_| s.next()).collect();

        let compute_maximum_length_from_left = |values: &Vec<u64>, wanted_sum: u64| {
            let mut maximum_length_from_left = vec![0; n];

            let mut active = BinaryHeap::new();
            let mut inactive = BinaryHeap::new();
            let mut sum = 0;
            for (i, &v) in values.iter().enumerate() {
                if sum + v <= wanted_sum {
                    active.push(v);
                    sum += v;
                } else {
                    inactive.push(Reverse(v));
                    while let Some(&Reverse(best_inactive_value)) = inactive.peek() {
                        match active.peek() {
                            None => {
                                assert!(sum + v > wanted_sum);
                                break;
                            }
                            Some(&worst_active_value) => {
                                if best_inactive_value < worst_active_value {
                                    active.pop();
                                    active.push(best_inactive_value);
                                    sum -= worst_active_value;

                                    inactive.pop();
                                    inactive.push(Reverse(worst_active_value));
                                    sum += best_inactive_value;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }

                maximum_length_from_left[i] = active.len();
            }

            maximum_length_from_left
        };

        let check_if_possible = |sum: u64| {
            let maximum_length_from_left = compute_maximum_length_from_left(&values, sum);

            let maximum_length_from_right: Vec<_> = {
                let reversed = values.clone().into_iter().rev().collect();
                compute_maximum_length_from_left(&reversed, sum)
                    .into_iter()
                    .rev()
                    .collect()
            };

            for i in 0..(n - 1) {
                if maximum_length_from_left[i] + maximum_length_from_right[i + 1] >= k {
                    return true;
                }
            }
            maximum_length_from_left[n - 1] >= k
        };

        let mut left = 0; // Never possible.
        let mut right = 10_u64.pow(16); // Always possible.

        while right - left > 1 {
            let middle = left + (right - left) / 2;
            if check_if_possible(middle) {
                right = middle;
            } else {
                left = middle;
            }
        }

        println!("{right}");
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
