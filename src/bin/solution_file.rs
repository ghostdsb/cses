use std::fmt::Debug;
use std::{io, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Edge{
    a: usize,
    b: usize,
    d: i64
}

impl Edge{
    fn new(a: usize, b: usize, d: i64) -> Self{
        Self { a, b, d }
    }
}

const MAX: i64 = 10_i64.pow(17); 
const MIN: i64 = -MAX; 

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);

    let mut distance: Vec<i64> = vec![MAX; n + 1];
    let mut edges: Vec<Edge> = vec![];
    for _ in 0..m {
        let input: Vec<i64> = input_vector(vec![]);
        let (a, b, d) = (input[0] as usize, input[1] as usize, input[2]);
        // converting single source longest path to single source shortest path 
        // by marking the distance negative
        edges.push(Edge::new(a, b, -d)) 
    }
    distance[1] = 0;
    for _ in 1..=n{
        for &edge in edges.iter(){
            let (a, b, d) = (edge.a, edge.b, edge.d);
            if distance[a] == MAX{continue;}
            distance[b] = std::cmp::min(distance[b], distance[a] + d); // relaxation
            distance[b] = std::cmp::max(distance[b], MIN); // reset to MIN if near MIN value
        } 
    }
    for _ in 1..=n{
        for &edge in edges.iter(){
            let (a, b, d) = (edge.a, edge.b, edge.d);
            if distance[a] == MAX{continue;}
            distance[b] = std::cmp::max(distance[b], MIN); // reset to MIN if near MIN value
            if distance[a] + d < distance[b]{
                distance[b] = MIN; // negative cycle detected hence, setting it to MIN
            }
        } 
    }
    // dbg!(distance);
    if distance[n] == MIN{
        println!("-1");
    }else{
        println!("{}", -distance[n]);
    }
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
