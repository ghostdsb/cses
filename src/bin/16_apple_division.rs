use cses::util::{input_single, input_vector};

fn main() {
    let n = input_single(0_usize);
    let apples: Vec<u64> = input_vector(vec![]);
    let ans = solution(n, apples);
    println!("{}", ans);
}

fn solution(n: usize, input: Vec<u64>) -> u64 {
    let total_sum: u64 = input.iter().sum();
    let mut ans = u64::MAX;
    for i in 0..(1 << n){
        let mut sum = 0;
        for j in 0..n{
            if i>>j & 1 == 1{
                sum += input[j];
                // print!("{} ", input[j]);
            }
        }
        // println!();
        // dbg!(sum);
        let rest = total_sum - sum;
        let diff = rest.abs_diff(sum);
        ans = ans.min(diff);
        // if sum <= total_sum/2{
        //     ans = ans.min(sum);
        // }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![934033764, 2, 7, 4, 1];

        let output = solution(input.len(), input);
        assert_eq!(output, 934033750);
    }

    #[test]
    fn it_works_2() {
        let input = vec![
            934033764, 747013925, 113297529, 621350044, 4858224, 896418401, 823418019, 490285020,
            811592918, 318107753, 122431099, 971962557, 68572395, 269437889, 506050802, 903504371,
            908615271, 406858603, 39392057, 816479348,
        ];

        let output = solution(input.len(), input);
        assert_eq!(output, 5483);
    }

    #[test]
    fn it_works_3() {
        let input = vec![1, 1];

        let output = solution(input.len(),input);
        assert_eq!(output, 0);
    }

    #[test]
    fn it_works_4() {
        let input = vec![1];

        let output = solution(input.len(), input);
        assert_eq!(output, 1);
    }
}
