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

