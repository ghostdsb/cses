use std::fmt::Debug;
use std::{io, str::FromStr};

use std::collections::{VecDeque, HashSet};
 
fn main() {
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
