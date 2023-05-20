fn main() {
    let mut s = Scanner::new();
    let n: usize = s.next();
    let m: usize = s.next();
    let d: i32 = s.next();

    let mut affected = vec![false; n + 1];
    (0..m).for_each(|_| {
        let x: usize = s.next();
        affected[x] = true;
    });

    let mut adj = vec![vec![]; n + 1];
    (1..n).for_each(|_| {
        let a: usize = s.next();
        let b: usize = s.next();

        adj[a].push(b);
        adj[b].push(a);
    });

    let mut max_distance_down: Vec<i32> = vec![-1; n + 1];
    let mut second_max_distance_down = vec![-1; n + 1];
    let mut solve_for_subtree = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for &neighbor in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);
        }

        let mut max = -1;
        let mut second_max = -1;
        for &neighbor in &adj[current] {
            if neighbor == parent {
                continue;
            }
            let dist = max_distance_down[neighbor];
            if dist != -1 {
                if dist >= max {
                    second_max = max;
                    max = dist;
                } else if dist >= second_max {
                    second_max = dist;
                }
            }
        }

        if max != -1 {
            max_distance_down[current] = max + 1;
        }

        if second_max != -1 {
            second_max_distance_down[current] = second_max + 1;
        }

        if affected[current] {
            if max_distance_down[current] == -1 {
                max_distance_down[current] = 0
            };
            if second_max_distance_down[current] == -1 {
                second_max_distance_down[current] = 0
            };
        }
    });

    solve_for_subtree.call(1, 0);

    let mut max_distance = vec![-1; n + 1];
    let mut second_max_distance = vec![-1; n + 1];
    let mut solve_for_all = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        if current == 1 {
            max_distance[current] = max_distance_down[current];
            second_max_distance[current] = second_max_distance_down[current];
        } else {
            let dist_from_parent = if max_distance[parent] != 1 + max_distance_down[current] {
                1 + max_distance[parent]
            } else if second_max_distance[parent] == -1 {
                -1
            } else {
                1 + second_max_distance[parent]
            };
            let mut possible_dists = vec![
                dist_from_parent,
                max_distance_down[current],
                second_max_distance_down[current],
            ];
            possible_dists.sort_unstable();
            max_distance[current] = possible_dists[2];
            second_max_distance[current] = possible_dists[1];
        }

        for &neighbor in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);
        }
    });
    solve_for_all.call(1, 0);

    let mut answer = 0;
    for node in 1..=n {
        if max_distance[node] <= d {
            answer += 1;
        }
    }

    println!("{}", answer);
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
