use std::fmt::Debug;
use std::{io, str::FromStr};

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

fn solve(n: usize, m: usize, grid: &Vec<Vec<char>>) {
    let mut starts: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        if grid[i][0] == '.' || grid[i][0] == 'A' {
            starts.push((i, 0));
        }
        if grid[i][m - 1] == '.' || grid[i][m - 1] == 'A' {
            starts.push((i, m - 1));
        }
    }
    for i in 1..(m - 1) {
        if grid[0][i] == '.' || grid[0][i] == 'A' {
            starts.push((0, i));
        }
        if grid[n - 1][i] == '.' || grid[n - 1][i] == 'A' {
            starts.push((n - 1, i));
        }
    }

    let dirs: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for &coord in starts.iter() {
        let mut q: VecDeque<(usize, (usize, usize))> = VecDeque::new();
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let mut path: Vec<(usize, (usize, usize))> = vec![];
        q.push_back((0, coord));
        while let Some((depth, (curr_i, curr_j))) = q.pop_front() {
            path.push((depth, (curr_i, curr_j)));
            set.insert((curr_i, curr_j));
            for &(dx, dy) in dirs.iter() {
                let nx = curr_i as i64 + dx;
                let ny = curr_j as i64 + dy;
                if nx >= 0 && ny >= 0 && nx < n as i64 && ny < m as i64 {
                    if grid[nx as usize][ny as usize] != '#'
                        && !set.contains(&(nx as usize, ny as usize))
                    {
                        set.insert((nx as usize, ny as usize));
                        q.push_back((depth + 1, (nx as usize, ny as usize)));
                    }
                }
            }
        }
    }
    println!("NO");
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
