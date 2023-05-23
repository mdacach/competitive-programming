use std::cmp::max;

fn main() {
    let mut scanner = Scanner::new();

    let (n, m) = (scanner.next(), scanner.next());
    let mut jiro_atk = vec![];
    let mut jiro_def = vec![];
    for _ in 0..n {
        let card_type: String = scanner.next();
        let strength: i32 = scanner.next();
        if card_type == "ATK" {
            jiro_atk.push(strength);
        } else {
            jiro_def.push(strength);
        }
    }

    let mut our_cards = vec![];
    for _ in 0..m {
        let strength: i32 = scanner.next();
        our_cards.push(strength);
    }

    jiro_atk.sort();
    our_cards.sort_by(|a, b| b.cmp(a));

    let mut answer = 0;
    'only_atk: for atk_cards_we_kill in 1..=jiro_atk.len() {
        let mut damage = 0;
        if our_cards.len() < atk_cards_we_kill {
            continue;
        }
        for i in 0..atk_cards_we_kill {
            let our = our_cards[i];
            let theirs = jiro_atk[i];
            if our < theirs {
                break 'only_atk;
            }
            damage += our - theirs;
        }
        answer = max(answer, damage);
    }

    // kill all cards
    // starting by DEF
    let cards_to_kill = jiro_def.len();
    if our_cards.len() < cards_to_kill {
        println!("{answer}");
        return;
    }

    let mut available_cards = our_cards.clone();
    'all_def: for their in &jiro_def {
        let chosen = available_cards
            .iter()
            .enumerate()
            .filter(|&(_, s)| *s > *their)
            .map(|(i, d)| (i, d - their))
            .min_by_key(|(_, d)| *d);

        match chosen {
            None => {
                available_cards.clear();
                break 'all_def;
            }
            Some((index, _card)) => {
                available_cards.remove(index);
            }
        }
    }

    if available_cards.is_empty() || available_cards.len() < jiro_atk.len() {
        println!("{answer}");
    } else {
        jiro_atk.sort_by(|a, b| b.cmp(a));
        let mut damage = 0;
        for i in 0..available_cards.len() {
            let our = available_cards[i];
            if i >= jiro_atk.len() {
                damage += our;
            } else {
                let other = jiro_atk[i];
                if our < other {
                    damage = 0;
                    break;
                }
                damage += max(0, our - other);
            }
        }
        answer = max(answer, damage);

        println!("{answer}");
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
