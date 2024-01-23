// Your task is to count the number of ways to construct sum n by throwing a dice one or more times. Each throw produces an outcome between 1 and  6.
// For example, if n=3, there are 4 ways:

// 1+1+1
// 1+2
// 2+1
// 3

use cses::util::input_single;
 
const MAX: u128 = 10_u128.pow(9) + 7;
 
fn main() {
    let input = input_single(0_u128);
    let mut d: Vec<i128> = vec![-1; (input + 1) as usize];
    let ans = solution(input as i64, &mut d);
    print!("{}", ans % MAX);
}
 
fn solution(n: i64, dp: &mut Vec<i128>) -> u128 {
    if n < 0 {
        return 0;
    }
 
    if n == 0 {
        return 1;
    }
 
    if dp[n as usize] != -1 {
        return dp[n as usize] as u128;
    }
 
    let mut ans = 0;
    for i in (1..=6).rev() {
        ans = (ans + solution(n - i, dp)) % MAX;
    }
    dp[n as usize] = ans as i128;
    return ans;
}