use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut max_length = 0;
    let mut count = 0;
    let mut last: char = '0';
    for c in input.chars() {
        if c == last {
            count += 1;
        } else {
            count = 0;
        }
        if count > max_length {
            max_length = count;
        }
        last = c;
    }
    println!("{}", max_length + 1);
}
