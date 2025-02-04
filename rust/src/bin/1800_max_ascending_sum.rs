// https://leetcode.com/problems/maximum-ascending-subarray-sum/
// 04/02/2025

// impl Solution {
//     pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
//         nums.windows(2)
//             .fold((nums[0], nums[0]), |(max_sum, curr_sum), w| {
//                 match w[0].cmp(&w[1]) {
//                     std::cmp::Ordering::Less => (max_sum.max(curr_sum + w[1]), curr_sum + w[1]),
//                     _ => (max_sum, w[1]),
//                 }
//             })
//             .0
//     }
// }

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0, 0), |(max_sum, curr_sum, prev), &num| {
                let new_sum = if num > prev { curr_sum + num } else { num };
                (max_sum.max(new_sum), new_sum, num)
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
    #[case([10, 20, 30, 5, 10, 50].to_vec(), 65)]
    #[case([10, 20, 30, 40, 50].to_vec(), 150)]
    #[case([12, 17, 15, 13, 10, 11, 12].to_vec(), 33)]
    fn test(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let result = Solution::max_ascending_sum(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([10, 20, 30, 5, 10, 50].to_vec(), 65),
        ([10, 20, 30, 40, 50].to_vec(), 150),
        ([12, 17, 15, 13, 10, 11, 12].to_vec(), 33),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::max_ascending_sum(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}
