use cses::util::{input_single, input_vector};

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m) = (input[0], input[1]);
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        let input: String = input_single(String::new());
        grid.push(input.chars().map(|c| c as char).collect());
    }
    let ans = solve(n, m, &mut grid);
    println!("{ans}");
}

fn solve(n: usize, m: usize, grid: &mut Vec<Vec<char>>) -> u64 {
    let dirs: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rooms = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '.' {
                rooms += 1;
                grid[i][j] = '#';
                let mut s: Vec<(usize, usize)> = vec![(i, j)];
                while !s.is_empty() {
                    let top = s.pop().unwrap();
                    for dr in dirs.iter() {
                        let nx = top.0 as i64 + dr.0;
                        let ny = top.1 as i64 + dr.1;
                        if nx >= 0
                            && ny >= 0
                            && (nx as i64) < (n as i64)
                            && (ny as i64) < (m as i64)
                        {
                            if grid[nx as usize][ny as usize] == '.' {
                                grid[nx as usize][ny as usize] = '#';
                                s.push((nx as usize, ny as usize));
                            }
                        }
                    }
                }
            }
        }
    }
    rooms
}
