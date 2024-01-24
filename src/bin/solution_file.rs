use std::fmt::Debug;
use std::{io, str::FromStr};

const MAX: u64 = 1e9 as u64 + 7;
fn main() {
    let n: u64 = input_single(0u64);
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..n{
        let l: String = input_single(String::new());
        grid.push(l.chars().collect::<Vec<char>>());
    }
    let x = solve(n, &grid);
    println!("{x}");
}

fn solve(n: u64, grid: &Vec<Vec<char>>) -> u64 {
    let mut memo: Vec<Vec<u64>> = vec![vec![0; n as usize]; n as usize];
    memo[0][0] = 1;
    for i in 0..memo.len(){
        for j in 0..memo[i].len(){
            if grid[i][j] == '.'{
                if i > 0 {
                    memo[i][j] = (memo[i][j] + memo[i-1][j]) % MAX;
                }
                if j > 0{
                    memo[i][j] = (memo[i][j] + memo[i][j-1]) % MAX;
                }
            }else{
                memo[i][j] = 0;
            }
        }
    }
    memo[n as usize -1][n as usize-1]
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
