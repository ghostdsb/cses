use cses::util::input_single;

fn main() {
    let input: u64 = input_single(0u64);
    let ans = solve(input);
    println!("{ans}");
}

fn solve(x: u64) -> u64 {
    // let mut memo: Vec<u64> = vec![0; x as usize +1];
    let mut x = x;
    let mut count = 0;
    while x != 0 {
        let m = max_digit(x) as u64;
        x -= m;
        // println!("{x} {m}");
        count += 1;
    }
    count
}

fn max_digit(x: u64) -> u8 {
    let mut y = x
        .to_string()
        .chars()
        .map(|d| d as u8 - '0' as u8)
        .collect::<Vec<u8>>();
    y.sort();
    // dbg!(&y);
    *y.last().unwrap()
}
