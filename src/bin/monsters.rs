// find all exits
// bfs from exits and check if player is closer

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

fn solve(n: usize, m: usize, grid: &Vec<Vec<char>>) {
    let mut starts: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        if grid[i][0] == '.' {
            starts.push((i, 0));
        }
        if grid[i][m - 1] == '.' {
            starts.push((i, m - 1));
        }
    }
    for i in 1..(m - 1) {
        if grid[0][i] == '.' {
            starts.push((0, i));
        }
        if grid[n - 1][i] == '.' {
            starts.push((n - 1, i));
        }
    }

    dbg!(starts);
}
