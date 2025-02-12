// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
// 12/02/2025

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut sum_arr = [0; 81];
        let mut result = -1;

        for num in nums {
            let sum_digit = Self::sum_digits(num) - 1;
            let n = sum_arr[sum_digit];

            if n > 0 {
                result = result.max(num + n);
            }

            sum_arr[sum_digit] = n.max(num);
        }

        result
    }

    #[inline(always)]
    fn sum_digits(num: i32) -> usize {
        let mut num = num;
        let mut result = 0;
        while num > 0 {
            result += (num % 10) as usize;
            num /= 10;
        }

        result
    }
}

// impl Solution {
//     pub fn maximum_sum(nums: Vec<i32>) -> i32 {
//         let mut summed: Vec<(i32, i32)> = nums
//             .iter()
//             .map(|&num| (Self::sum_digits(num), num))
//             .collect();

//         summed.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

//         summed.windows(2).fold(-1, |acc, w| {
//             if w[0].0 == w[1].0 {
//                 acc.max(w[0].1 + w[1].1)
//             } else {
//                 acc
//             }
//         })
//     }

//     #[inline(always)]
//     fn sum_digits(num: i32) -> i32 {
//         let mut num = num;
//         let mut result = 0;
//         while num > 0 {
//             result += num % 10;
//             num /= 10;
//         }

//         result
//     }
// }

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([18, 43, 36, 13, 7].to_vec(), 54)]
    #[case([10, 12, 19, 14].to_vec(), -1)]
    fn test(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let result = Solution::maximum_sum(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([18, 43, 36, 13, 7].to_vec(), 54),
        ([10, 12, 19, 14].to_vec(), -1),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::maximum_sum(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}
