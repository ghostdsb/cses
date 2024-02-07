use cses::util::input_vector;

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
    dbg!(&paths);
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

// fn solve(n: usize, a: &[usize], b: &[usize]) -> String {
//     let mut adj_vecs = vec![Vec::new(); n];
//     for i in 0..a.len() {
//         adj_vecs[a[i] - 1].push(b[i] - 1);
//         adj_vecs[b[i] - 1].push(a[i] - 1);
//     }

//     let mut nexts = vec![usize::MAX; n];
//     nexts[n - 1] = n - 1;

//     let mut queue = VecDeque::new();
//     queue.push_back(n - 1);

//     while let Some(head) = queue.pop_front() {
//         for &adj in &adj_vecs[head] {
//             if nexts[adj] == usize::MAX {
//                 nexts[adj] = head;
//                 queue.push_back(adj);
//             }
//         }
//     }

//     if nexts[0] == usize::MAX {
//         return String::from("IMPOSSIBLE");
//     }

//     let mut node = 0;
//     let mut route = vec![node + 1];
//     while node != n - 1 {
//         node = nexts[node];
//         route.push(node + 1);
//     }

//     format!(
//         "{}\n{}",
//         route.len(),
//         route
//             .iter()
//             .map(|x| x.to_string())
//             .collect::<Vec<_>>()
//             .join(" ")
//     )
// }
