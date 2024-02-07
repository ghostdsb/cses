use cses::util::input_vector;

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut visited: Vec<bool> = vec![false; n + 1];
    let mut bridges: Vec<usize> = vec![];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b) = (input[0], input[1]);
        adj[a].push(b);
        adj[b].push(a);
    }

    for i in 1..=n {
        if !visited[i] {
            bridges.push(i);
            dfs(i, &adj, &mut visited);
        }
    }
    dbg!(&bridges);
    println!("{}", bridges.len() - 1);
    for i in 1..bridges.len() {
        println!("{} {}", bridges[0], bridges[i]);
    }
}

fn dfs(node: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[node] = true;
    for n in adj[node].iter() {
        if !visited[*n] {
            dfs(*n, adj, visited);
        }
    }
}
