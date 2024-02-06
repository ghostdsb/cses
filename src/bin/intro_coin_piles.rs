// You have two coin piles containing a and b coins.
// On each move, you can either
// remove one coin from the left pile and two coins from the right pile,
// or two coins from the left pile and one coin from the right pile.
// Your task is to efficiently find out if you can empty both the piles.

// Input
// The first input line has an integer t: the number of tests.
// After this, there are t lines, each of which has two integers a and b: the numbers of coins in the piles.

// Output
// For each test, print "YES" if you can empty the piles and "NO" otherwise.
// Constraints

// 1 <= t <= 10^5
// 0 <= a, b <= 10^9

// Example
// Input:
// 3
// 2 1
// 2 2
// 3 3

// Output:
// YES
// NO
// YES

use std::fmt::Debug;
use std::{io, str::FromStr};
fn main() {
    let n = input_single(0_u32);
    for _ in 0..n {
        let coins: Vec<u32> = input_vector(vec![]);
        let x = (2 * coins[0]) as i32 - (coins[1]) as i32;
        let y = (2 * coins[1]) as i32 - (coins[0]) as i32;
        if coins[0] == 0 && coins[1] == 0 {
            println!("YES");
        } else if coins[0] > 0 && coins[1] > 0 && x >= 0 && y >= 0 && x % 3 == 0 && y % 3 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
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
