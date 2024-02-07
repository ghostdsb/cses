//brute forced as constraints low

use cses::util::input_vector;

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, m, q) = (input[0], input[1], input[2]);

    let mut distance: Vec<Vec<usize>> = vec![vec![usize::MAX; n + 2]; n + 2];

    for _ in 0..m {
        let input: Vec<usize> = input_vector(vec![]);
        let (a, b, d) = (input[0], input[1], input[2] as usize);
        distance[a][a] = 0;
        distance[b][b] = 0;
        distance[a][b] = std::cmp::min(distance[a][b], d);
        distance[b][a] = std::cmp::min(distance[a][b], d);
    }
    for k in 1..=n + 1 {
        for i in 1..=n + 1 {
            for j in 1..=n + 1 {
                if !(distance[i][k] == usize::MAX || distance[k][j] == usize::MAX) {
                    distance[i][j] = std::cmp::min(distance[i][j], distance[i][k] + distance[k][j]);
                }
            }
        }
    }
    for _ in 0..q {
        let input: Vec<usize> = input_vector(vec![]);
        let (start, end) = (input[0], input[1]);
        if distance[start][end] < usize::MAX {
            println!("{}", distance[start][end]);
        } else {
            println!("-1");
        }
    }
}
