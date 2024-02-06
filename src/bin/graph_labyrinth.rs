use cses::util::{input_single, input_vector};

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

use std::collections::VecDeque;
fn solve(n: usize, m: usize, grid: &mut Vec<Vec<char>>) {
    let dirs: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut path: VecDeque<(u64, (usize, usize))> = VecDeque::new();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'A' {
                path.push_back((0, (i, j)));
                break;
            }
        }
    }
    let mut x: Vec<(u64, (usize, usize))> = vec![];
    let mut distance = 0;
    let mut found = false;
    while !path.is_empty() && !found {
        let (depth, top) = path.pop_front().unwrap();
        x.push((depth, top));
        for dr in dirs.iter() {
            let nx = top.0 as i64 + dr.0;
            let ny = top.1 as i64 + dr.1;
            if nx >= 0 && ny >= 0 && (nx as i64) < (n as i64) && (ny as i64) < (m as i64) {
                if grid[nx as usize][ny as usize] == '.' {
                    grid[nx as usize][ny as usize] = '#';
                    path.push_back((depth + 1, (nx as usize, ny as usize)));
                }
                if grid[nx as usize][ny as usize] == 'B' {
                    grid[nx as usize][ny as usize] = '#';
                    path.push_back((depth + 1, (nx as usize, ny as usize)));
                    distance = depth + 1;
                    x.push((depth + 1, (nx as usize, ny as usize)));
                    found = true;
                    break;
                }
            }
        }
    }
    let (mut last_distance, mut end) = x.pop().unwrap();
    let mut ans_path = String::new();
    while !x.is_empty() {
        let (top_distance, coord) = x.pop().unwrap();
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

    if distance != 0 {
        println!("YES");
        println!("{distance}");
        println!("{:?}", ans_path.chars().rev().collect::<String>());
    } else {
        println!("NO");
    }
}
