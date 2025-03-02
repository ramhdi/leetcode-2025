// https://leetcode.com/problems/apply-operations-to-an-array/
// 01/03/2025

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        for i in 0..(n - 1) {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }

        let (mut i, mut j) = (0, 0);
        while j < n {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }

        while i < n {
            nums[i] = 0;
            i += 1;
        }

        nums
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1, 2, 2, 1, 1, 0].to_vec(), [1, 4, 2, 0, 0, 0].to_vec())]
    #[case([0, 1].to_vec(), [1, 0].to_vec())]
    fn test(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let result = Solution::apply_operations(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([1, 2, 2, 1, 1, 0].to_vec(), [1, 4, 2, 0, 0, 0].to_vec()),
        ([0, 1].to_vec(), [1, 0].to_vec()),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::apply_operations(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}
