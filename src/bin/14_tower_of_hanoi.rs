use cses::util::input_single;

fn main() {
    let input = input_single(String::new()).parse::<u8>().unwrap();
    let mut steps: Vec<(u8, u8)> = Vec::new();
    solution(input, 1, 2, 3, &mut steps);
    println!("{}", steps.len());
    steps.iter().for_each(|(a, c)| {
        println!("{} {}", a, c);
    });
}

fn solution(input: u8, a: u8, b: u8, c: u8, steps: &mut Vec<(u8, u8)>) {
    if input > 0 {
        solution(input - 1, a, c, b, steps);
        steps.push((a, c));
        solution(input - 1, b, a, c, steps);
    };
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let input = 2;
//         let output = solution(input);
//         assert_eq!(output, vec!["00","01","11","10"]);
//     }
// }
