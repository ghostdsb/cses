// single source shortest path algo (SSSP)
// dijkstra from 1st node using min-heap

use cses::util::{input_single, input_vector};

use std::collections::BinaryHeap;

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
    let mut dists: Vec<usize> = vec![usize::MAX; n + 1];

    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b, distance) = (input[0], input[1], input[2] as usize);
        adj[a].push(Node::new(b, distance));
    }
    // dbg!(&adj);
    dists[1] = 0;
    dijkstra(&adj, &mut dists);
    for i in 1..=n {
        print!("{} ", dists[i]);
    }
}

fn dijkstra(adj: &Vec<Vec<Node>>, dists: &mut Vec<usize>) {
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    heap.push(Node::new(1, 0));
    while !heap.is_empty() {
        dbg!(&heap);
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
}
