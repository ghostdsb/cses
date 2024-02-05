use std::fmt::Debug;
use std::{io, str::FromStr};

use std::collections::VecDeque;

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut grid: Vec<Vec<String>> = vec![];
    for _ in 0..n {
        let input: String = input_single(String::new());
        grid.push(input.chars().map(|c| c.to_string()).collect());
    }
    solve(n, m, &mut grid);
}

fn solve(n: usize, m: usize, grid: &mut Vec<Vec<String>>) {
    let dirs: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut q: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    let mut monsters: Vec<(usize, usize)> = vec![];
    let mut player: (usize, usize) = (0,0);
    for i in 0..n{
        for j in 0..m{
            if grid[i][j] == "M"{
                monsters.push((i,j));
                q.push_back((0, (i,j)));
            }
            if grid[i][j] == "A"{
                player = (i,j);
            }
        }
    }
    while !q.is_empty(){
        let (depth, (curr_i, curr_j)) = q.pop_front().unwrap();
        for dr in dirs.iter() {
            let nx = curr_i as i64 + dr.0;
            let ny = curr_j as i64 + dr.1;
            if nx >= 0 && ny >= 0 && (nx as i64) < (n as i64) && (ny as i64) < (m as i64) {
                if grid[nx as usize][ny as usize] == "."  {
                    grid[nx as usize][ny as usize] = format!("{}", depth+1);
                    q.push_back((depth + 1, (nx as usize, ny as usize)));
                }
            }
        }
    }

    q.push_back((0, player));
    let mut path: Vec<(usize, (usize, usize))> = vec![];
    while !q.is_empty(){
        let (depth, (curr_i, curr_j)) = q.pop_front().unwrap();
        path.push((depth, (curr_i, curr_j)));
        if curr_i == 0 || curr_i == n-1 || curr_j == 0 || curr_j == m-1{
            break;
        }
        for dr in dirs.iter() {
            let nx = curr_i as i64 + dr.0;
            let ny = curr_j as i64 + dr.1;
            if nx >= 0 && ny >= 0 && (nx as i64) < (n as i64) && (ny as i64) < (m as i64) {
                let grid_val = &grid[nx as usize][ny as usize]; 
                if grid_val == "." || grid_val.parse::<usize>().is_ok_and(|d| d > depth +1)  {
                    grid[nx as usize][ny as usize] = format!("{}", depth+1);
                    q.push_back((depth + 1, (nx as usize, ny as usize)));
                }
            }
        }
    }

    let (mut last_distance, mut end) = path.pop().unwrap();
    let mut ans_path = String::new();

    if !(end.0 == 0 || end.0 == n-1 || end.1 == 0 || end.1 == m-1){
        println!("NO");
        return;
    }

    while !path.is_empty() {
        let (top_distance, coord) = path.pop().unwrap();
        if top_distance == last_distance - 1 && end.0 == coord.0 && end.1.abs_diff(coord.1) == 1
            || end.1 == coord.1 && end.0.abs_diff(coord.0) == 1
        {
            if end.0 == coord.0 && end.1 > coord.1 {
                ans_path.push('R');
            }
            if end.0 == coord.0 && end.1 < coord.1 {
                ans_path.push('L');
            }
            if end.1 == coord.1 && end.0 > coord.0 {
                ans_path.push('D');
            }
            if end.1 == coord.1 && end.0 < coord.0 {
                ans_path.push('U');
            }
            last_distance = top_distance;
            end = coord;
        }
    }
    println!("YES");
    println!("{}",ans_path.len());
    println!("{}",ans_path.chars().rev().collect::<String>());

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
