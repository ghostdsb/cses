use std::fmt::Debug;
use std::{io, str::FromStr};

pub fn input_single<T>(default: T) -> T
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

pub fn input_vector<T>(default: Vec<T>) -> Vec<T>
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

use std::collections::VecDeque;
fn main(){
    let n: usize = input_single(0);
    let mut employees: Vec<usize> = vec![0; n+1];
    let boss: Vec<usize> = input_vector(vec![]);
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n+1];

    for (i, &e) in boss.iter().enumerate(){
        adj[e].push(i+2);
    }
    let mut visited: Vec<bool> = vec![false; n+1];
    q.push_back(1);
    let mut stack: Vec<usize> = vec![];
    while !q.is_empty(){
        let top = q.pop_front().unwrap();
        visited[top] = true;
        stack.push(top);
        for &s in adj[top].iter(){
            if !visited[s]{
                q.push_back(s);
            }
        }
    }
    while !stack.is_empty(){
        let emp = stack.pop().unwrap();
        if emp > 1{
            employees[boss[emp-2]] += employees[emp] + 1; 
        }
    }
    for i in 1..employees.len(){
        print!("{} ", employees[i]);
    }
}