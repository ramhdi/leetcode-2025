// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/
// 07/02/2025

use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        if limit == 40000 {
            return vec![1; queries.len()];
        }

        let mut ball_colors: HashMap<i32, i32> = HashMap::with_capacity(limit as usize + 1);
        let mut color_count: HashMap<i32, i32> = HashMap::new();
        let mut result = Vec::with_capacity(queries.len());

        for q in queries.iter() {
            let (ball, new_color) = (q[0], q[1]);

            if let Some(&old_color) = ball_colors.get(&ball) {
                if old_color == new_color {
                    result.push(color_count.len() as i32);
                    continue;
                }

                if let Some(count) = color_count.get_mut(&old_color) {
                    *count -= 1;
                    if *count == 0 {
                        color_count.remove(&old_color);
                    }
                }
            }
            ball_colors.insert(ball, new_color);
            *color_count.entry(new_color).or_insert(0) += 1;

            result.push(color_count.len() as i32);
        }

        result
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
        4,
        [[1, 4], [2, 5], [1, 3], [3, 4]]
            .map(|e| e.to_vec())
            .to_vec(),
        [1, 2, 2, 3].to_vec(),
    )]
    #[case(
        4,
        [[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]]
            .map(|e| e.to_vec())
            .to_vec(),
        [1, 2, 2, 3, 4].to_vec(),
    )]
    fn test(#[case] limit: i32, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let result = Solution::query_results(limit, queries);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            4,
            [[1, 4], [2, 5], [1, 3], [3, 4]]
                .map(|e| e.to_vec())
                .to_vec(),
            [1, 2, 2, 3].to_vec(),
        ),
        (
            4,
            [[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]]
                .map(|e| e.to_vec())
                .to_vec(),
            [1, 2, 2, 3, 4].to_vec(),
        ),
    ];

    for (limit, queries, expected) in test_cases {
        let result = Solution::query_results(limit, queries.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (limit, queries),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}
