fn main() {
    let mut scanner = Scanner::new();

    let n = scanner.next();
    let mut adj = vec![vec![]; n];
    (0..(n - 1)).for_each(|_| {
        let mut a: usize = scanner.next();
        let mut b: usize = scanner.next();
        a -= 1;
        b -= 1;

        adj[a].push((b, true));
        adj[b].push((a, false));
    });

    let mut needed_in = vec![0; n];
    let mut solve_for_in_subtree = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for &(child, this_direction) in &adj[current] {
            if child == parent {
                continue;
            }
            f.call(child, current);

            needed_in[current] += needed_in[child];
            if !this_direction {
                needed_in[current] += 1;
            }
        }
    });
    solve_for_in_subtree.call(0, 0);

    let mut needed_out = vec![0; n];
    let mut solve_for_out_subtree = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for &(child, this_direction) in &adj[current] {
            if child == parent {
                continue;
            }
            needed_out[child] = needed_out[current];
            needed_out[child] += needed_in[current] - needed_in[child];
            if !this_direction {
                needed_out[child] -= 1;
            }

            if this_direction {
                needed_out[child] += 1;
            }

            f.call(child, current);
        }
    });
    solve_for_out_subtree.call(0, 0);

    let answer = (0..n).map(|v| needed_in[v] + needed_out[v]).min().unwrap();
    let vertices = (0..n).filter(|&v| needed_in[v] + needed_out[v] == answer);
    println!("{answer}");
    vertices.for_each(|v| print!("{} ", v + 1));
    println!();
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
