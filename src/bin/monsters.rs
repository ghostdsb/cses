// find all exits
// bfs from exits and check if player is closer

use cses::util::{input_single, input_vector};
use std::collections::{HashSet, VecDeque};

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        let input: String = input_single(String::new());
        grid.push(input.chars().map(|c| c as char).collect());
    }
    solve(n, m, &mut grid);
}

fn solve(n: usize, m: usize, grid: &Vec<Vec<char>>) {}
