use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u64 = input.trim().parse().unwrap();

    print!("{} ", n);
    while n != 1 {
        if n & 1 == 1 {
            n = 3 * n + 1;
            print!("{} ", n);
        } else {
            n = n / 2;
            print!("{} ", n);
        }
    }

    io::stdout().flush().unwrap();
}
