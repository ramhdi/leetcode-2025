// https://leetcode.com/problems/number-of-ways-to-split-array/
// 03/01/2025

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let prefix_sum: Vec<i64> = nums
            .iter()
            .scan(0, |state, &num| {
                *state += num as i64;
                Some(*state)
            })
            .collect();
        let sum = prefix_sum[n - 1];

        prefix_sum
            .iter()
            .take(n - 1)
            .fold(0, |acc, &num| if num >= sum - num { acc + 1 } else { acc })
    }
}


#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        [10,4,-8,7].to_vec(),
        2
    )]
    #[case(
        [2,3,1,0].to_vec(),
        2
    )]
    #[case(
        [6, -1, 9].to_vec(), 
        0
    )]
    fn test_vowel_strings(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let result = Solution::ways_to_split_array(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([10, 4, -8, 7].to_vec(), 2),
        ([2, 3, 1, 0].to_vec(), 2),
        ([6, -1, 9].to_vec(), 0),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::ways_to_split_array(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}
