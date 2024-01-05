use std::fmt::Debug;
use std::io::{self, Write};
use std::str::FromStr;

fn input_single<T>(default: T) -> T
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse::<T>().unwrap(),
        Err(_) => default,
    }
}

fn main() {
    let n: u64 = input_single(0u64);
    if n == 2 || n == 3 {
        print!("NO SOLUTION");
    } else {
        let mut i = 2;
        while i <= n {
            print!("{} ", i);
            i += 2;
        }
        i = 1;
        while i <= n {
            print!("{} ", i);
            i += 2;
        }
    }
    io::stdout().flush().unwrap();
}
