use std::fmt::Debug;
use std::{io, str::FromStr};

use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    city: usize,
    distance: usize,
}

impl Node {
    fn new(city: usize, distance: usize) -> Self {
        Self { city, distance }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(&self))
    }
}
fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);

    let mut adj: Vec<Vec<Node>> = vec![vec![]; n + 1];

    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b, distance) = (input[0], input[1], input[2] as usize);
        adj[a].push(Node::new(b, distance));
    }
    // dbg!(&adj);
    let mut dists: Vec<usize> = vec![usize::MAX; n + 1];
    dists[1] = 0;

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    heap.push(Node::new(1, 0));
    while !heap.is_empty() {
        // dbg!(&heap);
        let top_node = heap.pop().unwrap();
        if dists[top_node.city] < top_node.distance {
            continue;
        }
        for &node in adj[top_node.city].iter() {
            if dists[node.city] <= top_node.distance + node.distance {
                continue;
            } else {
                dists[node.city] = top_node.distance + node.distance;
                heap.push(Node::new(node.city, top_node.distance + node.distance));
            }
        }
    }
    for i in 1..=n {
        print!("{} ", dists[i]);
    }
    // dbg!(dists);
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
