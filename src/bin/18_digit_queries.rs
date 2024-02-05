use cses::util::input_single;

fn main() {
    let n = input_single(0_usize);
    for _ in 0..n {
        let q = input_single(0_u64);
        let ans = solution(q);
        println!("{}", ans);
    }
}

// 1- 9  -> 9 numbers (1 digit numbers)
// 10-99 -> 90 numbers (2 digit numbers)
// 100-999 -> 900 numbers (3 digit numbers)
// Tn = 9 * 10^i * (i+1) where i -> [0..] (i+1 digit numbers)

fn solution(n: u64) -> u64 {
    if n < 10 {
        return n;
    }
    let mut i = 0;
    let mut sum = 0;
    let mut last_limit = 0;
    loop {
        let block = 9 * (i as u64 + 1) * 10_u64.pow(i);
        sum += block;
        if sum > n {
            sum -= block;
            break;
        }
        last_limit += 9 * 10_u64.pow(i);
        i += 1;
    }
    i -= 1;
    let last_nine = sum;
    let residue = n - last_nine;
    let nth_num = residue / (i as u64 + 2);
    let digit_index = residue % (i as u64 + 2);
    let mut num = last_limit + nth_num;
    let ans;
    if digit_index != 0 {
        num += 1;
        let digits = i + 2;
        let x = num / 10_u64.pow((digits as u64 - digit_index) as u32);
        ans = x % 10;
    } else {
        ans = num % 10;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let output = solution(7);
        assert_eq!(output, 7);
    }

    #[test]
    fn it_works_2() {
        let output = solution(19);
        assert_eq!(output, 4);
    }

    #[test]
    fn it_works_3() {
        let output = solution(12);
        assert_eq!(output, 1);
    }

    #[test]
    fn it_works_4() {
        let output = solution(158888888888888840);
        assert_eq!(output, 9);
    }

    #[test]
    fn it_works_5() {
        let output = solution(100_853_063_389_774_434);
        assert_eq!(output, 0);
    }
    #[test]
    fn it_works_6() {
        let output = solution(158888888888888842);
        assert_eq!(output, 9);
    }
}
