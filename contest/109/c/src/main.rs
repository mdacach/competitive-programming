fn main() {
    let mut s = Scanner::new();

    let n: usize = s.next();

    let is_lucky = |mut num: i32| {
        let mut digits = vec![];
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        !digits.iter().any(|&d| d != 4 && d != 7)
    };

    let mut adj = vec![vec![]; n + 1];
    (0..(n - 1)).for_each(|_| {
        let a: usize = s.next();
        let b: usize = s.next();
        let w: i32 = s.next();

        adj[a].push((b, is_lucky(w)));
        adj[b].push((a, is_lucky(w)));
    });

    let mut subtree_size = vec![1; n + 1];
    let mut compute_subtree_size = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for &(neighbor, _) in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);

            subtree_size[current] += subtree_size[neighbor];
        }
    });
    compute_subtree_size.call(1, 0);

    let mut lucky_nodes_in = vec![0; n + 1];
    let mut compute_lucky_nodes_in = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for &(neighbor, lucky) in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);

            if lucky {
                lucky_nodes_in[current] += subtree_size[neighbor];
            } else {
                lucky_nodes_in[current] += lucky_nodes_in[neighbor];
            }
        }
    });
    compute_lucky_nodes_in.call(1, 0);

    let mut lucky_nodes_out = vec![0; n + 1];
    let mut compute_lucky_nodes_out =
        RecursiveFunction2::new(|f, current: usize, parent: usize| {
            for &(neighbor, lucky) in &adj[current] {
                if neighbor == parent {
                    continue;
                }

                if lucky {
                    lucky_nodes_out[neighbor] = n as i64 - subtree_size[neighbor];
                } else {
                    lucky_nodes_out[neighbor] = lucky_nodes_out[current] + lucky_nodes_in[current]
                        - lucky_nodes_in[neighbor];
                }

                f.call(neighbor, current);
            }
        });
    compute_lucky_nodes_out.call(1, 0);

    let mut total_lucky_triplets: i64 = 0;

    let mut count_lucky_triplets = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        // choose from in
        total_lucky_triplets += max(0, lucky_nodes_in[current] * (lucky_nodes_in[current] - 1));

        // choose from out
        total_lucky_triplets += max(0, lucky_nodes_out[current] * (lucky_nodes_out[current] - 1));

        // from both
        total_lucky_triplets += max(0, lucky_nodes_in[current] * lucky_nodes_out[current] * 2);

        for &(neighbor, _) in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);
        }
    });
    count_lucky_triplets.call(1, 0);

    println!("{}", total_lucky_triplets);
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

use std::cmp::max;
use std::marker::PhantomData;

macro_rules! recursive_function {
    ($name: ident, $trait: ident, ($($type: ident $arg: ident,)*)) => {
        pub trait $trait<$($type, )*Output> {
            fn call(&mut self, $($arg: $type,)*) -> Output;
        }

        pub struct $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            f: std::cell::UnsafeCell<F>,
            $($arg: PhantomData<$type>,
            )*
            phantom_output: PhantomData<Output>,
        }

        impl<F, $($type, )*Output> $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            pub fn new(f: F) -> Self {
                Self {
                    f: std::cell::UnsafeCell::new(f),
                    $($arg: Default::default(),
                    )*
                    phantom_output: Default::default(),
                }
            }
        }

        impl<F, $($type, )*Output> $trait<$($type, )*Output> for $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            fn call(&mut self, $($arg: $type,)*) -> Output {
                unsafe { (*self.f.get())(self, $($arg, )*) }
            }
        }
    }
}

recursive_function!(RecursiveFunction0, Callable0, ());
recursive_function!(RecursiveFunction, Callable, (Arg arg,));
recursive_function!(RecursiveFunction2, Callable2, (Arg1 arg1, Arg2 arg2,));
recursive_function!(RecursiveFunction3, Callable3, (Arg1 arg1, Arg2 arg2, Arg3 arg3,));
recursive_function!(RecursiveFunction4, Callable4, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4,));
recursive_function!(RecursiveFunction5, Callable5, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5,));
recursive_function!(RecursiveFunction6, Callable6, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6,));
recursive_function!(RecursiveFunction7, Callable7, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6, Arg7 arg7,));
recursive_function!(RecursiveFunction8, Callable8, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6, Arg7 arg7, Arg8 arg8,));
recursive_function!(RecursiveFunction9, Callable9, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6, Arg7 arg7, Arg8 arg8, Arg9 arg9,));
