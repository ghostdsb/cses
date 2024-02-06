use std::fmt::Write;
use std::io;

fn main() {
    let number = input().trim().parse::<u64>().unwrap();
    let sol = print(solve(number));
    print!("{}", sol);
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input
}

fn solve(number: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    let mut n_sum: i64 = if number % 2 == 0 {
        (number / 2 * (number + 1)) as i64
    } else {
        (number * (number + 1) / 2) as i64
    };
    if n_sum % 2 == 1 {
        return None;
    } else {
        let mut vec1 = vec![];
        let mut vec2 = vec![];
        n_sum /= 2;
        let mut start = number as i64;
        while start > 0 {
            if n_sum as i64 - start as i64 >= 0 {
                vec1.push(start as u64);
                n_sum -= start;
            } else {
                vec2.push(start as u64);
            }
            start -= 1;
        }
        return Some((vec1, vec2));
    }
}

fn print(solution: Option<(Vec<u64>, Vec<u64>)>) -> String {
    match solution {
        None => "NO".to_string(),
        Some((v1, v2)) => {
            let mut buf = String::new();
            buf.push_str("YES\n");
            writeln!(buf, "{}", v1.len()).unwrap();
            writeln!(buf, "{}", space_sep(&v1)).unwrap();
            writeln!(buf, "{}", v2.len()).unwrap();
            writeln!(buf, "{}", space_sep(&v2)).unwrap();
            buf
        }
    }
}

fn space_sep<T: std::fmt::Display>(xs: &[T]) -> String {
    let mut res = String::new();
    for x in xs {
        if !res.is_empty() {
            res.push(' ');
        }
        write!(res, "{}", x).unwrap();
    }
    res
}
