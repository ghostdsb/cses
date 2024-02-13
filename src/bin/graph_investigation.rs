use cses::util::input_vector;

use std::collections::BinaryHeap;

const MAX: usize = 10_usize.pow(17);
const MOD: usize = 10_usize.pow(9) + 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node(usize, i64);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<Vec<Node>> = vec![vec![]; n + 1];
    let mut cost: Vec<i64> = vec![i64::MAX; n + 1];
    let mut routes: Vec<usize> = vec![0; n + 1];
    let mut min_flights: Vec<usize> = vec![MAX; n + 1];
    let mut max_flights: Vec<usize> = vec![0; n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b, c) = (input[0], input[1], input[2] as i64);
        adj[a].push(Node(b, c));
    }
    routes[1] = 1;
    cost[1] = 0;
    min_flights[1] = 0;
    dijkstra(
        &adj,
        &mut cost,
        &mut routes,
        &mut min_flights,
        &mut max_flights,
    );
    dbg!(&cost);
    dbg!(&routes);
    dbg!(&min_flights);
    dbg!(&max_flights);

    println!(
        "{} {} {} {}",
        cost[n], routes[n], min_flights[n], max_flights[n]
    );
}

fn dijkstra(
    adj: &Vec<Vec<Node>>,
    cost: &mut Vec<i64>,
    routes: &mut Vec<usize>,
    min_flights: &mut Vec<usize>,
    max_flights: &mut Vec<usize>,
) {
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut x: usize = 0;
    heap.push(Node(1, 0));
    while !heap.is_empty() {
        x += 1;
        dbg!(x);
        let Node(top, parent_cost) = heap.pop().unwrap();
        if cost[top] < parent_cost {
            continue;
        }
        for &child in adj[top].iter() {
            if cost[child.0] == child.1 + parent_cost {
                routes[child.0] += routes[top];
                routes[child.0] = routes[child.0] % MOD;
                min_flights[child.0] = std::cmp::min(min_flights[child.0], min_flights[top] + 1);
                max_flights[child.0] = std::cmp::max(max_flights[child.0], max_flights[top] + 1);
                cost[child.0] = child.1 + parent_cost;
                heap.push(Node(child.0, cost[child.0]));
            } else if cost[child.0] > child.1 + parent_cost {
                routes[child.0] = routes[top];
                min_flights[child.0] = min_flights[top] + 1;
                max_flights[child.0] = max_flights[top] + 1;
                cost[child.0] = child.1 + parent_cost;
                heap.push(Node(child.0, cost[child.0]));
            }
        }
    }
}
