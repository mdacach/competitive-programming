fn main() {
    const MOD: i64 = 998_244_353;

    let mut scanner = Scanner::new();

    let k = scanner.next();
    let mut games: Vec<Option<i64>> = (0..2_i32.pow(k))
        .map(|_| match scanner.next() {
            -1 => None,
            x => Some(x),
        })
        .collect();

    let factorial = |x| (1..=x).reduce(|acc, e| (acc * e) % MOD).unwrap_or(1);

    let mut answer = 1;

    while games.len() > 1 {
        let half = games.len() / 2;
        let mut next_round = vec![None; half];

        let players_losing_here = half as i64;
        let must_win = |player| player <= players_losing_here;

        games.chunks(2).enumerate().for_each(|(index, players)| {
            let first = players[0];
            let second = players[1];

            match (first, second) {
                (None, None) => {
                    answer *= 2;
                }
                (None, Some(x)) => {
                    if must_win(x) {
                        next_round[index] = Some(x);
                    }
                }
                (Some(x), None) => {
                    if must_win(x) {
                        next_round[index] = Some(x);
                    }
                }
                (Some(x), Some(y)) => {
                    if must_win(x) && must_win(y) {
                        answer = 0;
                    } else if must_win(x) {
                        next_round[index] = Some(x);
                    } else if must_win(y) {
                        next_round[index] = Some(y);
                    } else {
                        answer = 0;
                    }
                }
            }

            answer %= MOD;
        });

        let losing_fixed = games
            .iter()
            .filter_map(|&x| x)
            .filter(|&x| !must_win(x))
            .count() as i64;
        answer *= factorial(players_losing_here - losing_fixed);
        answer %= MOD;

        games = next_round;
    }

    println!("{answer}");
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
