// https://leetcode.com/problems/first-completely-painted-row-or-column/
// 20/01/2025

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut arr_pos = vec![(std::usize::MAX, std::usize::MAX); m * n + 1];
        for i in 0..m {
            for j in 0..n {
                arr_pos[mat[i][j] as usize] = (i, j);
            }
        }
        let (mut row_freq, mut col_freq) = (vec![0; m], vec![0; n]);

        for (i, num) in arr.iter().enumerate() {
            let (num_row, num_col) = arr_pos[*num as usize];
            row_freq[num_row] += 1;
            col_freq[num_col] += 1;
            if row_freq[num_row] == n || col_freq[num_col] == m {
                return i as i32;
            }
        }

        unreachable!()
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
        [1, 3, 4, 2].to_vec(),
        [[1, 4], [2, 3]].map(|e| e.to_vec()).to_vec(),
        2,
    )]
    #[case(
        [2, 8, 7, 4, 1, 3, 5, 6, 9].to_vec(),
        [[3, 2, 5], [1, 4, 6], [8, 7, 9]]
            .map(|e| e.to_vec())
            .to_vec(),
        3,
    )]
    fn test(#[case] arr: Vec<i32>, #[case] mat: Vec<Vec<i32>>, #[case] expected: i32) {
        let result = Solution::first_complete_index(arr, mat);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            [1, 3, 4, 2].to_vec(),
            [[1, 4], [2, 3]].map(|e| e.to_vec()).to_vec(),
            2,
        ),
        (
            [2, 8, 7, 4, 1, 3, 5, 6, 9].to_vec(),
            [[3, 2, 5], [1, 4, 6], [8, 7, 9]]
                .map(|e| e.to_vec())
                .to_vec(),
            3,
        ),
    ];

    for (arr, mat, expected) in test_cases {
        let result = Solution::first_complete_index(arr.clone(), mat.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (arr, mat),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}
