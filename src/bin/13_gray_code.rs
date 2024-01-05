use cses::util::input_single;

fn main() {
    let input = input_single(String::new()).parse::<u8>().unwrap();
    let ans = solution(input);
    ans.iter().for_each(|v| println!("{v}"));
}

fn solution(input: u8) -> Vec<String> {
    let mut ans = vec![0, 1];
    for _ in 0..(input - 1) {
        let initial_length = ans.len();
        for i in (0..ans.len()).rev() {
            ans.push(ans[i] + initial_length);
        }
    }
    let ans: Vec<String> = ans
        .iter()
        .map(|v| get_binary_string(*v as u16, input))
        .collect();
    // dbg!(get_binary_string(16, 16));
    // dbg!(ans);
    ans
}

fn get_binary_string(value: u16, pad: u8) -> String {
    match pad {
        1 => format!("{:b}", value),
        2 => format!("{:02b}", value),
        3 => format!("{:03b}", value),
        4 => format!("{:04b}", value),
        5 => format!("{:05b}", value),
        6 => format!("{:06b}", value),
        7 => format!("{:07b}", value),
        8 => format!("{:08b}", value),
        9 => format!("{:09b}", value),
        10 => format!("{:010b}", value),
        11 => format!("{:011b}", value),
        12 => format!("{:012b}", value),
        13 => format!("{:013b}", value),
        14 => format!("{:014b}", value),
        15 => format!("{:015b}", value),
        16 => format!("{:016b}", value),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = 2;
        let output = solution(input);
        assert_eq!(output, vec!["00", "01", "11", "10"]);
    }
}
