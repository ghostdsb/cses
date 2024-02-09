use cses::util::input_vector;

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut ins: Vec<usize> = vec![0; n + 1];
    let mut visited: Vec<bool> = vec![false; n + 1];
    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b) = (input[0], input[1]);
        adj[a].push(b);
        ins[b] += 1;
    }
    let mut stack: Vec<usize> = vec![];
    for i in 1..=n {
        if ins[i] == 0 {
            stack.push(i);
        }
    }
    let mut schedule: Vec<usize> = vec![];
    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        schedule.push(top);
        visited[top] = true;
        for &next in adj[top].iter() {
            if !visited[next] {
                ins[next] -= 1;
                if ins[next] == 0 {
                    stack.push(next);
                }
            }
        }
    }
    // dbg!(&schedule);
    if ins.iter().sum::<usize>() != 0 {
        println!("IMPOSSIBLE");
    } else {
        schedule.iter().for_each(|c| print!("{} ", c));
    }
}
