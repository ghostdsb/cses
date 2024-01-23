use cses::util::input_vector;

const MAX: u64 = 10_u64.pow(9) + 7;
fn main() {
    let input:Vec<u64> = input_vector(vec![]);
    let (_n,x) = (input[0], input[1]);
    let coins:Vec<u64> = input_vector(vec![]);
    let ans = solve(x, &coins);
    println!("{ans}");
}

fn solve(x: u64, coins: &Vec<u64>) -> u64 {
    let mut memo: Vec<u64> = vec![0; x as usize +1];
    memo[0] = 1;
    for i in 1..x+1{
        for coin in coins.iter(){
            if i >= *coin{
                memo[i as usize] = memo[i as usize] % MAX + memo[(i-coin) as usize] % MAX;
            }
        }
    }
    *memo.last().unwrap()
}
