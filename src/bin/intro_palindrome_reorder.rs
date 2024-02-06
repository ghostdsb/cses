use cses::util::input_single;

fn main() {
    let input = input_single(String::new());
    let ans = solution(input);
    println!("{ans}");
}

fn solution(input: String) -> String {
    use std::collections::HashMap;
    let mut ans: String = String::new();
    let mut hm: HashMap<char, u32> = HashMap::new();
    input.chars().for_each(|c| {
        hm.entry(c).and_modify(|count| *count += 1).or_insert(1);
    });
    let mut residue: char = ' ';
    let mut residue_count = 0;
    if hm.values().filter(|c| **c % 2 == 1).count() > 1 {
        "NO SOLUTION".to_string()
    } else {
        hm.iter().for_each(|(ch, count)| {
            if count % 2 == 0 {
                for _ in 0..count / 2 {
                    ans.push(*ch);
                }
            } else {
                residue = *ch;
                residue_count = *count;
            }
        });
        if residue != ' ' {
            let suffix = ans.chars().rev().collect::<String>();
            let suffix = &suffix.as_str();
            for _ in 0..residue_count {
                ans.push(residue);
            }
            ans += suffix;
        } else {
            let suffix = ans.chars().rev().collect::<String>();
            let suffix = &suffix.as_str();
            ans += suffix;
        }
        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "aab".to_string();
        let output = solution(input);
        assert_eq!(output, "aba");
    }
}
