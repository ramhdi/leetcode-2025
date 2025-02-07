// https://leetcode.com/problems/tuple-with-same-product/
// 06/02/2025

use std::collections::HashMap;

// impl Solution {
//     pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
//         let n = nums.len();
//         let mut product_freq = HashMap::<i32, i32>::with_capacity((n * (n - 1)) >> 1);

//         for i in 0..n {
//             for j in (i + 1)..n {
//                 *product_freq.entry(nums[i] * nums[j]).or_insert(0) += 1;
//             }
//         }

//         let mut result = 0;
//         for &f in product_freq.values() {
//             if f >= 2 {
//                 result += 4 * f * (f - 1);
//             }
//         }

//         result
//     }
// }

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let product_freq = (0..n).fold(
            HashMap::<i32, i32>::with_capacity((n * (n - 1)) >> 1),
            |mut acc, i| {
                ((i + 1)..n).for_each(|j| *acc.entry(nums[i] * nums[j]).or_insert(0) += 1);
                acc
            },
        );

        product_freq
            .values()
            .filter_map(|&f| if f >= 2 { Some(4 * f * (f - 1)) } else { None })
            .sum()
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([2, 3, 4, 6].to_vec(), 8)]
    #[case([1, 2, 4, 5, 10].to_vec(), 16)]
    #[case([1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192].to_vec(), 1288)]
    fn test(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let result = Solution::tuple_same_product(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![([2, 3, 4, 6].to_vec(), 8), ([1, 2, 4, 5, 10].to_vec(), 16)];

    for (nums, expected) in test_cases {
        let result = Solution::tuple_same_product(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}
