use std::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let n = input_single(0_usize);
    for _ in 0..n{
        let q = input_single(0_u64);
        let ans = solution(q);
        println!("{}", ans);
    }
}

fn solution(n: u64) -> u64 {
    if n < 10{
        return n
    }
    let mut i = 0;
    let mut sum = 0;
    let mut last_limit = 0;
    loop {
        let block = 9 * (i as u64+1) * 10_u64.pow(i);
        sum += block ;
        if sum > n{
            sum -= block;
            break;
        }
        last_limit += 9 * 10_u64.pow(i);
        i+=1;
    }
    i-=1;
    let last_nine = sum;
    let residue = n - last_nine;
    let nth_num = residue / (i as u64+2);
    let digit_index = residue % (i as u64 +2);
    let mut num = last_limit + nth_num;
    if digit_index != 0{
        num += 1;
        let digits = i + 2;
        let x = num / 10_u64.pow((digits as u64-digit_index) as u32);
        return x % 10;
    }else{
        return num % 10;
    }
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
