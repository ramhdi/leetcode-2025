// https://leetcode.com/problems/letter-tile-possibilities/
// 17/02/2025

// DFS + recursion: 0 ms, 2.3 MB
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut char_count = tiles.bytes().fold([0; 26], |mut acc, c| {
            acc[(c - b'A') as usize] += 1;
            acc
        });

        let mut result = 0;
        Self::backtrack(&mut char_count, &mut result);

        result
    }

    fn backtrack(char_count: &mut [i32; 26], result: &mut i32) {
        for i in 0..26 {
            if char_count[i] != 0 {
                *result += 1;
                char_count[i] -= 1;
                Self::backtrack(char_count, result);
                char_count[i] += 1;
            }
        }
    }
}

// BFS + deque: 0 ms, 3.2 MB
// impl Solution {
//     pub fn num_tile_possibilities(tiles: String) -> i32 {
//         use std::collections::VecDeque;

//         let char_count = tiles.bytes().fold([0; 26], |mut acc, c| {
//             acc[(c - b'A') as usize] += 1;
//             acc
//         });

//         let mut result = 0;
//         let mut queue = VecDeque::new();
//         queue.push_back(char_count.clone());

//         while let Some(current_count) = queue.pop_front() {
//             for i in 0..26 {
//                 if current_count[i] > 0 {
//                     result += 1;
//                     let mut next_count = current_count.clone();
//                     next_count[i] -= 1;
//                     queue.push_back(next_count);
//                 }
//             }
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
    #[case("AAB".to_string(), 8)]
    #[case("AAABBC".to_string(), 188)]
    #[case("V".to_string(), 1)]
    fn test(#[case] tiles: String, #[case] expected: i32) {
        let result = Solution::num_tile_possibilities(tiles);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ("AAB".to_string(), 8),
        ("AAABBC".to_string(), 188),
        ("V".to_string(), 1),
    ];

    for (tiles, expected) in test_cases {
        let result = Solution::num_tile_possibilities(tiles.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            tiles, expected, result
        );
        assert_eq!(result, expected);
    }
}
