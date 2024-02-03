use std::fmt::Debug;
use std::{io, str::FromStr};

use std::collections::VecDeque;
fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut visited: Vec<bool> = vec![false; n + 1];
    let mut colors: Vec<u8> = vec![0; n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b) = (input[0], input[1]);
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut q: VecDeque<(u8, usize, usize)> = VecDeque::new();

    let mut tree: Vec<(u8, usize, usize)> = vec![];
    for i in 1..=n {
        if !visited[i] {
            q.push_back((1, 0, i));
        }
        while !q.is_empty() {
            let (color, depth, top) = q.pop_front().unwrap();
            visited[top] = true;
            tree.push((color, depth, top));
            if colors[top] == 0 {
                colors[top] = color;
            } else if colors[top] != color {
                println!("IMPOSSIBLE");
                return;
            }
            for &node in adj[top].iter() {
                if !visited[node] {
                    let mut c = 1;
                    if color == 1 {
                        c = 2;
                    }
                    q.push_back((c, depth + 1, node));
                }
            }
        }
    }
    // dbg!(&tree);

    // dbg!(&colors);
    for i in 1..=n {
        print!("{} ", colors[i]);
    }
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
