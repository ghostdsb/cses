use cses::util::{input_single, input_vector};
use std::collections::HashMap;

fn main() {
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut grid: Vec<Vec<bool>> = vec![vec![false; n + 1]; n + 1];
    let mut visited: Vec<bool> = vec![false; n + 1];

    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b) = (input[0], input[1]);
        grid[a][b] = true;
        grid[b][a] = true;
        adj.entry(a).and_modify(|l| l.push(b)).or_insert(vec![b]);
        adj.entry(b).and_modify(|l| l.push(a)).or_insert(vec![a]);
    }

    let mut g: Vec<usize> = vec![];
    for i in 1..=n {
        if !visited[i] {
            let mut stack: Vec<usize> = vec![];
            // let mut s: Vec<usize> = vec![];
            // s.push(i);
            visited[i] = true;
            stack.push(i);

            while !stack.is_empty() {
                let top = stack.pop().unwrap();
                visited[top] = true;

                if let Some(list) = adj.get(&top) {
                    for j in list {
                        if !visited[*j] && *j != top && grid[top][*j] {
                            stack.push(*j);
                        }
                    }
                }
            }
            g.push(i);
        }
    }
    // dbg!(g);
    println!("{}", g.len() - 1);
    for i in 1..g.len() {
        println!("{} {}", g[0], g[i]);
    }
}
