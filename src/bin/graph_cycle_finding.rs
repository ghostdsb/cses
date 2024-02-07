// bellman ford

use cses::util::input_vector;

#[derive(Debug, Clone, Copy)]
struct Edge {
    a: usize,
    b: usize,
    d: i64,
}

impl Edge {
    fn new(a: usize, b: usize, d: i64) -> Self {
        Self { a, b, d }
    }
}

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);

    let mut distance: Vec<i64> = vec![0; n + 1];
    let mut edges: Vec<Edge> = vec![];
    for _ in 0..m {
        let input: Vec<i64> = input_vector(vec![]);
        let (a, b, d) = (input[0] as usize, input[1] as usize, input[2]);
        // converting single source longest path to single source shortest path
        // by marking the distance negative
        edges.push(Edge::new(a, b, d))
    }
    distance[1] = 0;
    let mut parents: Vec<Option<usize>> = vec![None; n + 1];
    let mut x: Option<usize> = None;

    // compute relaxation
    for _ in 1..=n {
        x = None;
        for &edge in edges.iter() {
            let (a, b, d) = (edge.a, edge.b, edge.d);
            if distance[a] + d < distance[b] {
                distance[b] = distance[a] + d; // relaxation
                x = Some(b);
                parents[b] = Some(a);
            }
        }
    }
    if x.is_none() {
        println!("NO");
    } else {
        println!("YES");
        for _ in 0..n {
            x = parents[x.unwrap()];
        }
        let mut cycle: Vec<usize> = vec![x.unwrap()];
        let mut curr = x;
        while parents[curr.unwrap()] != x {
            // print!("{} ",curr);
            curr = parents[curr.unwrap()];
            cycle.push(curr.unwrap());
        }
        cycle.reverse();
        cycle.push(cycle[0]);
        cycle.iter().for_each(|n| print!("{} ", n));
    }
    // dbg!(x);
    // dbg!(parents);
}
