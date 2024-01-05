use std::io;

fn main() {
    let number = input().trim().parse::<u64>().unwrap();
    let sol = solve(number);
    print!("{}", sol);
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input
}

fn solve(n: u64) -> u64 {
    let mut count = 0;
    let mut n = n;
    while n > 0 {
        n /= 5;
        count += n;
    }
    count
}
