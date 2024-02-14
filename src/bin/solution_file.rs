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
    let n:usize = input_single(0);
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n+1];
    for _ in 0..n-1{
        let input: Vec<usize> = input_vector(vec![]);
        let (a,b) = (input[0], input[1]);
        adj[a].push(b);
        adj[b].push(a);
    }
    for i in 1..=n{
        let d = distance(&adj, i);
        print!("{} ", d);
    }
}

fn distance(adj: &Vec<Vec<usize>>, start: usize) -> usize{
    let n = adj.len()-1;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n+1];
    q.push_back((0, start));
    let mut max_distance: usize = 0;
    while !q.is_empty(){
        let (distance, top) = q.pop_front().unwrap();
        visited[top] = true;
        max_distance = distance;
        for &n in adj[top].iter(){
            if !visited[n]{
                q.push_back((distance + 1, n));
            }
        }
    }
    max_distance
}