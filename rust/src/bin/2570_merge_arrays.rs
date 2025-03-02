// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/
// 02/03/2025

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut id_val = [0; 1001];
        let (n1, n2) = (nums1.len(), nums2.len());
        let mut result = Vec::with_capacity(n1 + n2);

        for p in nums1 {
            id_val[p[0] as usize] += p[1];
        }

        for p in nums2 {
            id_val[p[0] as usize] += p[1];
        }

        for i in 0..1001 {
            if id_val[i] > 0 {
                result.push(vec![i as i32, id_val[i]]);
            }
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
        [[1, 2], [2, 3], [4, 5]].map(|e| e.to_vec()).to_vec(),
        [[1, 4], [3, 2], [4, 1]].map(|e| e.to_vec()).to_vec(),
        [[1, 6], [2, 3], [3, 2], [4, 6]]
            .map(|e| e.to_vec())
            .to_vec(),
    )]
    #[case(
        [[2, 4], [3, 6], [5, 5]].map(|e| e.to_vec()).to_vec(),
        [[1, 3], [4, 3]].map(|e| e.to_vec()).to_vec(),
        [[1, 3], [2, 4], [3, 6], [4, 3], [5, 5]]
            .map(|e| e.to_vec())
            .to_vec(),
    )]
    fn test(
        #[case] nums1: Vec<Vec<i32>>,
        #[case] nums2: Vec<Vec<i32>>,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        let result = Solution::merge_arrays(nums1, nums2);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            [[1, 2], [2, 3], [4, 5]].map(|e| e.to_vec()).to_vec(),
            [[1, 4], [3, 2], [4, 1]].map(|e| e.to_vec()).to_vec(),
            [[1, 6], [2, 3], [3, 2], [4, 6]]
                .map(|e| e.to_vec())
                .to_vec(),
        ),
        (
            [[2, 4], [3, 6], [5, 5]].map(|e| e.to_vec()).to_vec(),
            [[1, 3], [4, 3]].map(|e| e.to_vec()).to_vec(),
            [[1, 3], [2, 4], [3, 6], [4, 3], [5, 5]]
                .map(|e| e.to_vec())
                .to_vec(),
        ),
    ];

    for (nums1, nums2, expected) in test_cases {
        let result = Solution::merge_arrays(nums1.clone(), nums2.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (nums1, nums2),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}
