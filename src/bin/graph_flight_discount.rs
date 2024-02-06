use cses::util::{input_single, input_vector};

struct Edge {
    a: usize,
    b: usize,
    c: usize,
}

impl Edge {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }
}

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut egdes: Vec<Edge> = vec![];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b, c) = (input[0], input[1], input[2]);
        egdes.push(Edge::new(a, b, c));
    }
}
