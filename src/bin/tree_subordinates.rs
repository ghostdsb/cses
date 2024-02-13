use cses::util::{input_single, input_vector};

use std::collections::VecDeque;
fn main(){
    let n: usize = input_single(0);
    let mut employees: Vec<usize> = vec![0; n+1];
    let mut boss: Vec<usize> = input_vector(vec![]);
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n+1];

    for (i, &e) in boss.iter().enumerate(){
        adj[e].push(i+2);
    }
    let mut visited: Vec<bool> = vec![false; n+1];
    q.push_back(1);
    let mut stack: Vec<usize> = vec![];
    while !q.is_empty(){
        let top = q.pop_front().unwrap();
        visited[top] = true;
        stack.push(top);
        for &s in adj[top].iter(){
            if !visited[s]{
                q.push_back(s);
            }
        }
    }
    dbg!(&adj);
    dbg!(&stack);
    while !stack.is_empty(){
        let emp = stack.pop().unwrap();
        dbg!(emp);
        if emp > 1{
            employees[boss[emp-2]] += employees[emp] + 1; 
        }
    }
    dbg!(&employees);
    for i in 1..employees.len(){
        print!("{} ", employees[i]);
    }
}