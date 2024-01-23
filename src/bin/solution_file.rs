use std::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let input: u64 = input_single(0u64);
    let ans = solve(input);
    println!("{ans}");
}

fn solve(x: u64) -> u64 {
    let mut x = x;
    let mut count = 0;
    while x != 0{
        let m = max_digit(x) as u64;
        x -= m;
        count += 1;
    }
    count
}

fn max_digit(x: u64) -> u8{
    let mut y = x.to_string().chars().map(|d| d as u8 - '0' as u8).collect::<Vec<u8>>();
    y.sort();
    *y.last().unwrap()
}


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

fn input_vector<T>(default: Vec<T>) -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input
            .as_str()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
        Err(_) => default,
    }
}
