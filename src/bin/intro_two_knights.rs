use std::fmt::Debug;
use std::io;
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

fn placement_ways(n: i64) -> i64 {
    n * n * (n * n - 1) / 2 - 4 * (n - 3 + 1) * (n - 2 + 1)
}

fn main() {
    let n: i32 = input_single(0i32);
    for i in 1..n + 1 {
        let ans = placement_ways(i.into());
        println!("{}", ans);
    }
}
