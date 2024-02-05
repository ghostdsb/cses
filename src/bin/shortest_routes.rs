// single source shortest path algo (SSSP)

use cses::util::{input_single, input_vector};

use std::collections::HashMap;
fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<HashMap<usize, u64>> = vec![HashMap::new(); n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b, distance) = (input[0], input[1], input[2] as u64);
        // adj[a].push((b, distance));
        adj[a]
            .entry(b)
            .and_modify(|d| {
                if *d > distance {
                    *d = distance;
                }
            })
            .or_insert(distance);
    }
    dbg!(&adj);
    let mut dists: Vec<u64> = vec![u64::MAX; n + 1];
    dists[1] = 0;
    let mut stack: Vec<(u64, usize)> = vec![(0, 1)];
    while !stack.is_empty() {
        let (d, top) = stack.pop().unwrap();
        for (&neighbour, &distance) in adj[top].iter() {
            if distance + d < dists[neighbour] {
                dists[neighbour] = distance + d;
                stack.push((distance + d, neighbour));
            }
        }
    }
    for i in 1..=n {
        print!("{} ", dists[i]);
    }
    // dbg!(dists);
}
