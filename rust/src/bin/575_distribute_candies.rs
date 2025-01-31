// https://leetcode.com/problems/distribute-candies/

use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len();
        let candy_set: HashSet<i32> = candy_type.into_iter().collect();
        (n / 2).min(candy_set.len()) as i32
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1, 1, 2, 2, 3, 3].to_vec(), 3)]
    #[case([1, 1, 2, 3].to_vec(), 2)]
    #[case([6, 6, 6, 6].to_vec(), 1)]
    fn test(#[case] candy_type: Vec<i32>, #[case] expected: i32) {
        let result = Solution::distribute_candies(candy_type);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([1, 1, 2, 2, 3, 3].to_vec(), 3),
        ([1, 1, 2, 3].to_vec(), 2),
        ([6, 6, 6, 6].to_vec(), 1),
    ];

    for (candy_type, expected) in test_cases {
        let result = Solution::distribute_candies(candy_type.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            candy_type, expected, result
        );
        assert_eq!(result, expected);
    }
}
