use cses::util::input_vector;

use std::collections::BinaryHeap;
fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b) = (input[0], input[1]);
        adj[a].push((b, -1));
    }

    let mut distances: Vec<(usize, i64)> = vec![(0, i64::MAX); n + 1];
    distances[1] = (0, 0);

    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    heap.push((0, 1));

    while !heap.is_empty() {
        let top = heap.pop().unwrap();
        if distances[top.1].1 < top.0 {
            continue;
        }
        for &child in adj[top.1].iter() {
            if distances[child.0].1 > distances[top.1].1 + child.1 {
                distances[child.0] = (top.1, distances[top.1].1 + child.1);
                heap.push((distances[top.1].1 + child.1, child.0));
            }
        }
    }

    let mut curr = n;
    let mut path: Vec<usize> = vec![curr];
    while curr != 0 {
        curr = distances[curr].0;
        path.push(curr);
    }
    if path.len() >= 3 && path[path.len() - 2] == 1 {
        println!("{} ", path.len() - 1);
        for i in 0..path.len() - 1 {
            print!("{} ", path[path.len() - 2 - i]);
        }
        return;
    }
    println!("IMPOSSIBLE")
}
