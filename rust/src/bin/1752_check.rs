// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/
// 02/02/2025

// impl Solution {
//     pub fn check(nums: Vec<i32>) -> bool {
//         let n = nums.len();
//         if n <= 1 {
//             return true;
//         }

//         (0..nums.len())
//             .map(|start| (start..start + n - 1).all(|i| nums[i % n] <= nums[(i + 1) % n]))
//             .any(|c| c)
//     }
// }

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        (0..n).filter(|&i| nums[i] > nums[(i + 1) % n]).count() <= 1
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([3, 4, 5, 1, 2].to_vec(), true)]
    #[case([2, 1, 3, 4].to_vec(), false)]
    #[case([1, 2, 3].to_vec(), true)]
    fn test(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let result = Solution::check(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([3, 4, 5, 1, 2].to_vec(), true),
        ([2, 1, 3, 4].to_vec(), false),
        ([1, 2, 3].to_vec(), true),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::check(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}
