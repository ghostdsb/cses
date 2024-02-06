use std::fmt::Debug;
use std::{io, str::FromStr};

use std::collections::BinaryHeap;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
    city: usize,
    cost: usize,
}

impl Edge {
    fn new(city: usize, cost: usize) -> Self {
        Self { city, cost }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HeapNode {
    cost: usize,
    city: usize,
    flag: bool,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(&self))
    }
}

impl HeapNode {
    fn new(city: usize, cost: usize, flag: bool) -> Self {
        Self { city, cost, flag }
    }
}

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut distance_full: Vec<usize> = vec![usize::MAX; n + 1];
    let mut distance_disc: Vec<usize> = vec![usize::MAX; n + 1];
    let mut adj: Vec<Vec<Edge>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b, c) = (input[0], input[1], input[2]);
        adj[a].push(Edge::new(b, c));
    }
    distance_full[1] = 0;
    distance_disc[1] = 0;
    dijkstra(&adj, &mut distance_full , &mut distance_disc );
    println!("{}", distance_disc[n]);
}

fn dijkstra(adj: &Vec<Vec<Edge>>, distance_full: &mut Vec<usize>, distance_disc: &mut Vec<usize>) {
    let mut heap: BinaryHeap<HeapNode> = BinaryHeap::new();
    heap.push(HeapNode::new(1, 0, false));
    while !heap.is_empty() {
        // dbg!(&heap);
        let top_node = heap.pop().unwrap();
        if top_node.flag && distance_disc[top_node.city] < top_node.cost{
            continue;
        }
        if !top_node.flag && distance_full[top_node.city] < top_node.cost{
            continue;
        }
        for &conn in adj[top_node.city].iter() {
            if top_node.flag{
                if distance_disc[conn.city] > conn.cost + top_node.cost{
                    distance_disc[conn.city] = conn.cost + top_node.cost;
                    heap.push(HeapNode::new(conn.city, conn.cost + top_node.cost, true));
                }
            }else{
                if distance_full[conn.city] > conn.cost + top_node.cost{
                    distance_full[conn.city] = conn.cost + top_node.cost;
                    heap.push(HeapNode::new(conn.city, conn.cost + top_node.cost, false));
                }
                if distance_disc[conn.city] > conn.cost/2 + top_node.cost{
                    distance_disc[conn.city] = conn.cost/2 + top_node.cost;
                    heap.push(HeapNode::new(conn.city, conn.cost/2 + top_node.cost, true));
                }

            }
        }
    }
}

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
