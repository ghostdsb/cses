use std::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b) = (input[0], input[1]);
        adj[a].push(b);
        adj[b].push(a);
    }
    // dbg!(&adj);
    let mut visited: Vec<bool> = vec![false; n + 1];
    let mut parents: Vec<usize> = vec![0; n + 1];
    for city in 1..=n {
        if !visited[city] {
            let mut stack: Vec<usize> = vec![city];
            while let Some(top) = stack.pop() {
                visited[top] = true;
                for &child in adj[top].iter() {
                    if !visited[child] {
                        stack.push(child);
                        parents[child] = top;
                    } else if visited[child] && child != parents[top] {
                        parents[child] = top;
                        let mut curr = child;
                        let start = curr;
                        let mut circuit: Vec<usize> = vec![];
                        // print!("{} ", start);
                        circuit.push(start);
                        loop {
                            // print!("{} ", parents[curr]);
                            circuit.push(parents[curr]);
                            curr = parents[curr];
                            if curr == start {
                                break;
                            }
                        }
                        println!("{}", circuit.len());
                        for cir in circuit.iter() {
                            print!("{} ", cir);
                        }
                        return;
                    }
                }
            }
        }
    }
    println!("IMPOSSIBLE")
}

// fn input_single<T>(default: T) -> T
// where
//     T: FromStr + Debug,
//     <T as FromStr>::Err: Debug,
// {
//     let mut input = String::new();
//     match io::stdin().read_line(&mut input) {
//         Ok(_) => input.trim().parse::<T>().unwrap(),
//         Err(_) => default,
//     }
// }

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
