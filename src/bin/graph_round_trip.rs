// cycle detection
// dfs from each unvisited node
// keep track of parents of nodes
// if an visited node encountered with different parent; cycle detected
use cses::util::input_vector;

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
                        let mut circut: Vec<usize> = vec![];
                        // print!("{} ", start);
                        circut.push(start);
                        loop {
                            // print!("{} ", parents[curr]);
                            circut.push(parents[curr]);
                            curr = parents[curr];
                            if curr == start {
                                break;
                            }
                        }
                        println!("{}", circut.len());
                        for cir in circut.iter() {
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
