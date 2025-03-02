// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/
// 25/02/2025

const MOD: i32 = 1_000_000_000 + 7;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        arr.iter()
            .scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            })
            .fold((0, 0, 0), |(result, ces, cos), prefix_sum| {
                match prefix_sum % 2 {
                    0 => ((result + cos) % MOD, ces + 1, cos),
                    1 => ((result + ces + 1) % MOD, ces, cos + 1),
                    _ => unreachable!(),
                }
            })
            .0
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1, 3, 5].to_vec(), 4)]
    #[case([2, 4, 6].to_vec(), 0)]
    #[case([1, 2, 3, 4, 5, 6, 7].to_vec(), 16)]
    fn test(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let result = Solution::num_of_subarrays(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([1, 3, 5].to_vec(), 4),
        ([2, 4, 6].to_vec(), 0),
        ([1, 2, 3, 4, 5, 6, 7].to_vec(), 16),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::num_of_subarrays(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}
