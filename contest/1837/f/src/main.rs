use std::cmp::min;
use std::thread::current;

fn main() {
    let mut s = Scanner::new();
    let t = s.next();
    'testcase: for _ in 0..t {
        let (n, k): (usize, usize) = (s.next(), s.next());
        let values: Vec<u64> = (0..n).map(|_| s.next()).collect();

        // Find the k-th smallest.
        let sorted = {
            let mut v = values.clone();
            v.sort();
            v
        };

        let big = sorted[k - 1];
        let big_count = values.iter().filter(|&&x| x == big).count() as u64;
        let big_prefix_sum: Vec<u64> = {
            let mut res = vec![];
            let ones = values
                .iter()
                .map(|&x| if x == big { 1 } else { 0 })
                .reduce(|acc, e| {
                    res.push(acc + e);
                    acc + e
                })
                .unwrap();
            res
        };
        let bigs_in_range = |l: usize, r: usize| {
            if l == 0 {
                big_prefix_sum[r]
            } else {
                big_prefix_sum[r] - big_prefix_sum[l - 1]
            }
        };

        // These guys are part of the solution.
        let fixed: Vec<(usize, u64)> = sorted
            .into_iter()
            .enumerate()
            .take_while(|(i, x)| *x < big)
            .collect();
        assert!(fixed.len() <= k);

        // But now we need to choose k - fixed, from big_count
        let must_choose = (k - fixed.len()) as u64;

        let mut answer = u64::MAX;
        let mut current_prefix_sum = 0;
        let total_sum: u64 = fixed.iter().map(|(_i, x)| x).sum();
        for values_in_prefix in 0..fixed.len() {
            let suffix_sum = total_sum - current_prefix_sum;

            let mut bigs_available_for_prefix_only = {
                let position_last_prefix = fixed[values_in_prefix].0;
                bigs_in_range(0, position_last_prefix)
            };

            let mut bigs_available_for_suffix_only = {
                let position_last_suffix = if values_in_prefix + 1 < fixed.len() {
                    fixed[values_in_prefix + 1].0
                } else {
                    0
                };
                bigs_in_range(position_last_suffix, n - 1)
            };

            let mut bigs_available_for_both =
                { big_count - (bigs_available_for_prefix_only + bigs_available_for_suffix_only) };

            if suffix_sum > current_prefix_sum {
                // We will put `big` in prefix until prefix + big > suffix.
                let delta = suffix_sum - current_prefix_sum;
                let mut can_put = delta / big;

                if can_put >= must_choose {
                    let going_to_put = must_choose; // We can only put that much.

                    let maximum_here = bigs_available_for_prefix_only + bigs_available_for_both;
                    if maximum_here >= going_to_put {
                        // We are able to put them all.
                        // In the end, suffix is still greater, so our answer is the suffix portion anyway.
                        // So we don't actually need to perform the operations here.
                        answer = min(answer, suffix_sum);
                    } else {
                        // We can only put `maximum_here`.
                        let remaining = going_to_put - maximum_here;
                        // These will need to be put on the suffix portion.
                        answer = min(answer, suffix_sum + remaining * big);
                    }
                } else {
                    // We are able to put all we need on the prefix, but we still need to put more in order
                    // to end with `k` total numbers.
                    let going_to_put = can_put;
                    let maximum_here = bigs_available_for_prefix_only + bigs_available_for_both;

                    if maximum_here >= going_to_put {
                        // We are able to put them all.
                        let mut new_pref = current_prefix_sum + going_to_put * big;
                        // Some of those came from the prefix portion and some from the both portion.
                        if bigs_available_for_prefix_only < going_to_put {
                            bigs_available_for_both -=
                                going_to_put - bigs_available_for_prefix_only;
                            bigs_available_for_prefix_only = 0;
                        }

                        // Afterwards, we need to start alternating between prefix and suffix until we have enough values.

                        let left_needed = must_choose - going_to_put;
                        let min_both = min(
                            bigs_available_for_prefix_only,
                            bigs_available_for_suffix_only,
                        );
                        if min_both * 2 >= left_needed {
                            // All good, alternate.
                            // Answer will come from suffix anyway.
                            answer = min(answer, suffix_sum + min_both * big);
                        } else {
                            // One of them is 0, and then you need to pick from `both`.
                            let new_pref = new_pref + min_both * big;
                            let new_suf = suffix_sum + min_both * big;
                            let left_needed = left_needed - 2 * min_both;
                        }

                        answer = min(answer, suffix_sum);
                    } else {
                        // We can only put `maximum_here`.
                        let remaining = going_to_put - maximum_here;
                        // These will need to be put on the suffix portion.
                        answer = min(answer, suffix_sum + remaining * big);
                    }
                }

                // 1. Put `big` so that the next one would make prefix too big.
                if bigs_available_for_prefix_only >= can_put {
                    bigs_available_for_prefix_only -= can_put;
                    can_put = 0;

                    // And now we should alternate between prefix and suffix.
                }

                // Afterwards, we alternate putting on prefix and suffix (if possible, how?)
            } else {
                // We will put `big` in suffix until suffix + big > prefix.
                let delta = current_prefix_sum - suffix_sum;
                let can_put = delta / big;
                // TODO: will_put is also bounded by how many are in the correct positions.
                let will_put = min(can_put, big_count);

                // Afterwards, we alternate putting on suffix and prefix (if possible, how?)
            }

            // This element will be considered as part of the prefix in the next iteration.
            current_prefix_sum += fixed[values_in_prefix].1;
        }

        println!("***");
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
