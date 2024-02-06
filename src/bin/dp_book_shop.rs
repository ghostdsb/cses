//book_shop

use cses::util::input_vector;

fn main() {
    let input: Vec<usize> = input_vector(vec![]);
    let (n, x) = (input[0], input[1]);
    let book_prices: Vec<usize> = input_vector(vec![]);
    let pages: Vec<usize> = input_vector(vec![]);
    let ans = solve(n, x, book_prices, pages);
    println!("{ans}");
}

fn solve(n: usize, x: usize, book_prices: Vec<usize>, pages: Vec<usize>) -> usize {
    // let mut memo: Vec<Vec<u128>> = vec![vec![0; x as usize + 1]; n as usize + 1];
    // for i in 1..=n {
    //     for j in 0..=x {
    //         memo[i as usize][j as usize] = memo[i as usize - 1][j as usize];
    //         let left: i128 = j as i128 - book_prices[i as usize - 1] as i128;
    //         if left >= 0 {
    //             memo[i as usize][j as usize] = memo[i as usize][j as usize]
    //                 .max(memo[i as usize - 1][left as usize] + pages[i as usize - 1]);
    //         }
    //     }
    // }
    // memo[n as usize][x as usize]

    let mut dp: Vec<usize> = vec![0; 100001];
    for i in 0..n {
        for j in (book_prices[i]..=x).rev() {
            dp[j] = std::cmp::max(dp[j], pages[i] + dp[j - book_prices[i]]);
        }
    }
    dp[x]
}
