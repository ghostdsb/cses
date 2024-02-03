use std::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut visited: Vec<bool> = vec![false; n + 1];
    let mut found = false;
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b) = (input[0], input[1]);
        adj[a].push(b);
        adj[b].push(a);
    }

    use std::collections::VecDeque;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((0, 1));
    let mut paths: Vec<(usize, usize, usize)> = vec![];
    for i in 1..=n {
        if !visited[i] {
            while !q.is_empty() && !found {
                let (depth, top) = q.pop_front().unwrap();
                // paths.push((depth, i, top));
                visited[top] = true;
                for node in adj[top].iter() {
                    if !visited[*node] {
                        q.push_back((depth + 1, *node));
                        paths.push((depth + 1, top, *node));
                        visited[*node] = true;
                        if *node == n {
                            found = true;
                            // paths.push((depth + 1, top, *node));
                            break;
                        }
                    }
                }
            }
        }
    }
    // dbg!(&paths);
    if !found {
        println!("IMPOSSIBLE");
    } else {
        let (mut ans, mut parent, dest) = paths.pop().unwrap();
        let mut sol_path: Vec<usize> = vec![dest];
        while !paths.is_empty() {
            let (d, par, top) = paths.pop().unwrap();
            if top == parent && d == ans - 1 {
                sol_path.push(top);
                parent = par;
                ans = d;
            }
        }
        sol_path.push(1);
        println!("{}", sol_path.len());
        for p in sol_path.iter().rev() {
            print!("{} ", p);
        }
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
