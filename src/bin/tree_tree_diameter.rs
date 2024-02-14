use cses::util::{input_single, input_vector};
use std::collections::VecDeque;
fn main(){
    let n:usize = input_single(0);
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n+1];
    for _ in 0..n-1{
        let input: Vec<usize> = input_vector(vec![]);
        dbg!(&input);
        let (a,b) = (input[0], input[1]);
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n+1];
    q.push_back((0, 1));
    let mut last = 0;
    while !q.is_empty(){
        let (distance, top) = q.pop_front().unwrap();
        visited[top] = true;
        last = top;
        for &n in adj[top].iter(){
            if !visited[n]{
                q.push_back((distance + 1, n));
            }
        }
    }
    q.push_back((0,last));
    let mut dia = 0;
    visited = vec![false; n+1];
    while !q.is_empty(){
        dbg!(&q);
        let (distance, top) = q.pop_front().unwrap();
        visited[top] = true;
        dia = distance;
        for &n in adj[top].iter(){
            if !visited[n]{
                q.push_back((distance + 1, n));
            }
        }
    }
    print!("{}", dia);
}