// single source K shortest paths

use cses::util::input_vector;
use std::collections::BinaryHeap;
#[derive(Debug, Clone, Copy)]
struct Edge {
    city: usize,
    cost: usize,
}

impl Edge {
    fn new(city: usize, cost: usize) -> Self {
        Self { city, cost }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    city: usize,
    cost: usize,
}

impl Node {
    fn new(city: usize, cost: usize) -> Self {
        Self { city, cost }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m, k) = (input[0], input[1], input[2]);
    let mut adjacent: Vec<Vec<Edge>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b, c) = (input[0], input[1], input[2]);
        adjacent[a].push(Edge::new(b, c));
    }
    dbg!(&adjacent);
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; k]; n + 1];
    dijkstra(&adjacent, &mut dist);
    dbg!(&dist);
    dist[n].iter().for_each(|d| print!("{} ", d));
}

fn dijkstra(adj: &Vec<Vec<Edge>>, dist: &mut Vec<Vec<usize>>) {
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    heap.push(Node::new(1, 0));
    let k = dist[0].len();
    while !heap.is_empty() {
        dbg!(&heap);
        let top_node = heap.pop().unwrap();
        if dist[top_node.city][k - 1] < top_node.cost {
            continue;
        }
        for node in adj[top_node.city].iter() {
            if dist[node.city][k - 1] > node.cost + top_node.cost {
                dist[node.city][k - 1] = node.cost + top_node.cost;
                dist[node.city].sort();
                heap.push(Node::new(node.city, node.cost + top_node.cost))
            }
        }
    }
}
