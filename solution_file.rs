use std::fmt::Debug;
use std::{io, str::FromStr};

use std::collections::{VecDeque, HashSet};
 
fn main() {
<<<<<<< HEAD
    let input:Vec<u32> = input_vector(vec![]);
    let (_n,x) = (input[0], input[1]);
    let coins:Vec<u32> = input_vector(vec![]);
    let ans = solution(x, coins);
    println!("{}", ans);
}
 
fn solution(x: u32, coins: Vec<u32>) -> u32 {
    let mut q: VecDeque<(u32, u32)> = VecDeque::new();
    q.push_back((0, x));
    while !q.is_empty(){
        let (d, top) = q.pop_front().unwrap();
        if top == 0{
            return d;
        }
        for i in coins.iter(){
            if top as i32 - *i as i32 >= 0 {
                q.push_back((d+1, top-i));
            }
        }
    }
    return 0
=======
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
>>>>>>> 27ea451eeebd6641d933e9794b4049f1706accc2
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
