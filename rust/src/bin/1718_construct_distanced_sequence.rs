// https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/
// 16/02/2025

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let n_usize = n as usize;
        let size = 2 * n_usize - 1;
        let mut arr = vec![0; size];
        let mut counts = vec![0; n_usize + 1];
        Self::backtrack(&mut arr, &mut counts, 0, n_usize).unwrap()
    }

    fn backtrack(
        arr: &mut Vec<i32>,
        counts: &mut Vec<usize>,
        pos: usize,
        n: usize,
    ) -> Option<Vec<i32>> {
        if pos == arr.len() {
            return Some(arr.clone());
        }
        if arr[pos] != 0 {
            return Self::backtrack(arr, counts, pos + 1, n);
        }
        for k in (1..=n).rev() {
            if k == 1 {
                if counts[k] == 0 {
                    arr[pos] = 1;
                    counts[k] += 1;
                    let res = Self::backtrack(arr, counts, pos + 1, n);
                    if res.is_some() {
                        return res;
                    }
                    arr[pos] = 0;
                    counts[k] -= 1;
                }
            } else {
                if counts[k] != 0 {
                    continue;
                }
                let next_pos = pos + k;
                if next_pos >= arr.len() {
                    continue;
                }
                if arr[next_pos] != 0 {
                    continue;
                }
                arr[pos] = k as i32;
                arr[next_pos] = k as i32;
                counts[k] = 2;
                let res = Self::backtrack(arr, counts, pos + 1, n);
                if res.is_some() {
                    return res;
                }
                arr[pos] = 0;
                arr[next_pos] = 0;
                counts[k] = 0;
            }
        }
        None
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3, [3, 1, 2, 3, 2].to_vec())]
    #[case(5, [5, 3, 1, 4, 3, 5, 2, 4, 2].to_vec())]
    fn test(#[case] n: i32, #[case] expected: Vec<i32>) {
        let result = Solution::construct_distanced_sequence(n);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (3, [3, 1, 2, 3, 2].to_vec()),
        (5, [5, 3, 1, 4, 3, 5, 2, 4, 2].to_vec()),
    ];

    for (n, expected) in test_cases {
        let result = Solution::construct_distanced_sequence(n);
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            n, expected, result
        );
        assert_eq!(result, expected);
    }
}
