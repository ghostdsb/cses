// Consider a money system consisting of n coins. Each coin has a positive integer value. Your task is to produce a sum of money x using the available coins in such a way that the number of coins is minimal.
// For example, if the coins are \{1,5,7\} and the desired sum is 11, an optimal solution is 5+5+1 which requires 3 coins.

use cses::util::input_vector;

 
fn main() {
    let input:Vec<u32> = input_vector(vec![]);
    let (n,x) = (input[0], input[1]);
    let coins:Vec<u32> = input_vector(vec![]);
    let ans = solve(n, x, &coins);
    if ans == u32::MAX{
        println!("-1");
    }else{
        println!("{ans}");
    }
}

fn solve(n: u32, x: u32, coins: &Vec<u32>) -> u32 {
    let mut memo: Vec<u32> = vec![u32::MAX; x as usize +1];
    memo[0] = 0;
    for i in 1..=n{
        for j in 1..=x{
            if j >= coins[i as usize - 1]{
                let x: u64 = memo[(j-coins[i as usize - 1]) as usize] as u64 + 1;
                if x > u32::MAX as u64{
                    memo[j as usize] = memo[j as usize];
                }else{
                    memo[j as usize] = memo[j as usize].min(memo[(j-coins[i as usize - 1]) as usize] + 1);
                }
            }
        }
    }
    memo[x as usize]
}
