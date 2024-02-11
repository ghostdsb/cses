use cses::util::input_vector;
use std::collections::VecDeque;
const MOD: usize = 10_usize.pow(9) + 7;
fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut in_degree: Vec<usize> = vec![0; n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b) = (input[0], input[1]);
        adj[a].push(b);
        in_degree[b] += 1;
    }
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 2..=n {
        if in_degree[i] == 0 {
            q.push_back(i);
        }
    }
    while !q.is_empty() {
        let top = q.pop_front().unwrap();
        for &child in adj[top].iter() {
            in_degree[child] -= 1;
            if in_degree[child] == 0 && child != 1 {
                q.push_back(child);
            }
        }
    }
    let mut paths: Vec<usize> = vec![0; n + 1];
    q.push_back(1);
    paths[1] = 1;
    while !q.is_empty() {
        let top = q.pop_front().unwrap();
        for &child in adj[top].iter() {
            in_degree[child] -= 1;
            paths[child] = (paths[child] + paths[top]) % MOD;
            if in_degree[child] == 0 {
                q.push_back(child);
            }
        }
    }
    dbg!(&in_degree);
    dbg!(&adj);
    dbg!(&q);
    dbg!(&paths);
    println!("{}", paths[n]);
}
