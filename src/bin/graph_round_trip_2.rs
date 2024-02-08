use cses::util::input_vector;

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b) = (input[0], input[1]);
        adj[a].push(b);
    }
    dbg!(&adj);
    let mut visited: Vec<bool> = vec![false; n + 1];
    let mut marked: Vec<bool> = vec![false; n + 1];
    let mut stack: Vec<usize> = vec![];
    visit(&adj, &mut stack, &mut marked, &mut visited);
    if stack.is_empty() {
        println!("IMPOSSIBLE");
        return;
    }
    let start = stack.last().unwrap();
    let mut can_start = false;
    let mut ans: Vec<usize> = vec![];
    for &node in stack.iter() {
        if node == *start {
            can_start = true;
        }
        if can_start {
            ans.push(node);
        }
    }
    println!("{}", ans.len());
    ans.iter().for_each(|node| print!("{} ", node));
}

fn visit(
    adj: &Vec<Vec<usize>>,
    stack: &mut Vec<usize>,
    marked: &mut Vec<bool>,
    visited: &mut Vec<bool>,
) {
    let n: usize = visited.len();
    for i in 1..n {
        if !visited[i] {
            if dfs(adj, stack, marked, visited, i) {
                break;
            }
        }
    }
}

fn dfs(
    adj: &Vec<Vec<usize>>,
    stack: &mut Vec<usize>,
    marked: &mut Vec<bool>,
    visited: &mut Vec<bool>,
    node: usize,
) -> bool {
    marked[node] = true;
    visited[node] = true;
    stack.push(node);
    dbg!(&stack);
    for &child in adj[node].iter() {
        if !visited[child] {
            if dfs(adj, stack, marked, visited, child) {
                return true;
            }
        }
        if marked[child] {
            stack.push(child);
            return true;
        }
    }
    stack.pop();
    marked[node] = false;
    false
}
