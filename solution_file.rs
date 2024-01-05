use std::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let n = input_single(0_usize);
    let apples: Vec<u64> = input_vector(vec![]);
    let ans = solution(n, apples);
    println!("{}", ans);
}

fn solution(n: usize, input: Vec<u64>) -> u64 {
    let total_sum: u64 = input.iter().sum();
    let mut ans = u64::MAX;
    for i in 0..(1 << n){
        let mut sum = 0;
        for j in 0..n{
            if i>>j & 1 == 1{
                sum += input[j];
            }
        }
        let rest = total_sum - sum;
        let diff = rest.abs_diff(sum);
        ans = ans.min(diff);
    }
    ans
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
