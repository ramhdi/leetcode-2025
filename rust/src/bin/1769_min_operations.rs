// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/
// 06/01/2025

// impl Solution {
//     pub fn min_operations(boxes: String) -> Vec<i32> {
//         let boxes = boxes.as_bytes();
//         let n = boxes.len();
//         let mut result = vec![0; n];

//         for i in 0..n {
//             let mut moves = 0;
//             for j in 0..n {
//                 moves += if boxes[j] == b'1' {
//                     i.abs_diff(j) as i32
//                 } else {
//                     0
//                 };
//             }
//             result[i] = moves;
//         }

//         result
//     }
// }

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.as_bytes();
        let n = boxes.len();
        let mut result = vec![0; n];

        // First pass: left to right
        let mut balls = 0;
        let mut moves = 0;
        for i in 0..n {
            result[i] += moves;
            if boxes[i] == b'1' {
                balls += 1;
            }
            moves += balls;
        }

        // Second pass: right to left
        balls = 0;
        moves = 0;
        for i in (0..n).rev() {
            result[i] += moves;
            if boxes[i] == b'1' {
                balls += 1;
            }
            moves += balls;
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
    #[case("110".to_string(), [1, 1, 3].to_vec())]
    #[case("001011".to_string(), [11, 8, 5, 4, 3, 4].to_vec())]
    fn test(#[case] boxes: String, #[case] expected: Vec<i32>) {
        let result = Solution::min_operations(boxes);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ("110".to_string(), [1, 1, 3].to_vec()),
        ("001011".to_string(), [11, 8, 5, 4, 3, 4].to_vec()),
    ];

    for (boxes, expected) in test_cases {
        let result = Solution::min_operations(boxes.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            boxes, expected, result
        );
        assert_eq!(result, expected);
    }
}
