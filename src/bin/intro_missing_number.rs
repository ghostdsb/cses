use std::io::{self};

fn main() {
    let mut input_n = String::new();
    let n: u32 = match io::stdin().read_line(&mut input_n) {
        Ok(_) => input_n.trim().parse().unwrap(),
        Err(_) => 0,
    };

    let mut input_nums = String::new();
    let nums: Vec<i32> = match io::stdin().read_line(&mut input_nums) {
        Ok(_) => input_nums
            .as_str()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
        Err(_) => vec![],
    };

    let mut idx_xor = 0;
    let mut num_xor = 0;
    for i in 1..n + 1 {
        idx_xor = idx_xor ^ (i);
        if i < n {
            num_xor ^= nums[i as usize - 1];
        }
    }
    println!("{}", (idx_xor as i32) ^ num_xor);
}
