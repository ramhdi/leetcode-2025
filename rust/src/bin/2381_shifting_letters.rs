// https://leetcode.com/problems/shifting-letters-ii/
// 05/01/2025

// Time limit exceeded!
// impl Solution {
//     pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
//         let n = s.len();
//         let shifts: Vec<i32> = shifts.iter().fold(vec![0; n], |mut acc, s| {
//             let (start, end, dir) = (s[0] as usize, s[1] as usize, s[2]);
//             for i in start..=end {
//                 acc[i] += if dir == 1 { 1 } else { -1 };
//             }
//             acc
//         });

//         // println!("{:?}", shifts);

//         s.bytes()
//             .zip(shifts.iter())
//             .map(|(b, s)| {
//                 let old_byte = (b - b'a') as i32;
//                 let new_byte = ((old_byte + s) % 26 + 26) % 26;
//                 (new_byte as u8 + b'a') as char
//             })
//             .collect()
//     }
// }

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n = s.len();

        let diff = shifts.iter().fold(vec![0; n + 1], |mut acc, s| {
            let (start, end, dir) = (s[0] as usize, s[1] as usize, s[2]);
            let val = if dir == 1 { 1 } else { -1 };
            acc[start] += val;
            acc[end + 1] -= val;
            acc
        });

        let shifts: Vec<i32> = diff[..n]
            .iter()
            .scan(0, |curr_shift, &d| {
                *curr_shift += d;
                Some(*curr_shift)
            })
            .collect();

        s.bytes()
            .zip(shifts.iter())
            .map(|(b, &shift)| {
                let pos = (b - b'a') as i32;
                let new_pos = ((pos + shift) % 26 + 26) % 26;
                (new_pos as u8 + b'a') as char
            })
            .collect()
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
        "abc".to_string(),
        [[0,1,0],[1,2,1],[0,2,1]].map(|e| e.to_vec()).to_vec(),
        "ace".to_string()
    )]
    #[case(
        "dztz".to_string(),
        [[0,0,0],[1,1,1]].map(|e| e.to_vec()).to_vec(),
        "catz".to_string()
    )]
    fn test(#[case] s: String, #[case] shifts: Vec<Vec<i32>>, #[case] expected: String) {
        let result = Solution::shifting_letters(s, shifts);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            "abc".to_string(),
            [[0, 1, 0], [1, 2, 1], [0, 2, 1]]
                .map(|e| e.to_vec())
                .to_vec(),
            "ace".to_string(),
        ),
        (
            "dztz".to_string(),
            [[0, 0, 0], [1, 1, 1]].map(|e| e.to_vec()).to_vec(),
            "catz".to_string(),
        ),
    ];

    for (s, shifts, expected) in test_cases {
        let result = Solution::shifting_letters(s.clone(), shifts.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            s, expected, result
        );
        assert_eq!(result, expected);
    }
}
