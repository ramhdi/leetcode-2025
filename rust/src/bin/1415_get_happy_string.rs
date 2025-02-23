// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
// 19/02/2025

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let (n, k) = (n as usize, k as usize);
        let mut happy_strings = vec![vec![b'a'], vec![b'b'], vec![b'c']];
        Self::generate_happy_strings(&mut happy_strings, n);
        String::from_utf8(happy_strings.get(k - 1).unwrap_or(&vec![]).to_owned()).unwrap()
    }

    fn generate_happy_strings(strings: &mut Vec<Vec<u8>>, n: usize) {
        if strings[0].len() == n {
            return;
        }

        let mut new_strings: Vec<Vec<u8>> = Vec::with_capacity(strings.len() * 2);
        for s in strings.iter() {
            let (mut s1, mut s2) = (s.clone(), s.clone());
            match s.last() {
                Some(b'a') => {
                    (s1.push(b'b'), s2.push(b'c'));
                }
                Some(b'b') => {
                    (s1.push(b'a'), s2.push(b'c'));
                }
                Some(b'c') => {
                    (s1.push(b'a'), s2.push(b'b'));
                }
                _ => unreachable!(),
            }
            (new_strings.push(s1), new_strings.push(s2));
        }
        *strings = new_strings;
        Self::generate_happy_strings(strings, n);
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 3, "c".to_string())]
    #[case(1, 4, "".to_string())]
    #[case(3, 9, "cab".to_string())]
    fn test(#[case] n: i32, #[case] k: i32, #[case] expected: String) {
        let result = Solution::get_happy_string(n, k);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (1, 3, "c".to_string()),
        (1, 4, "".to_string()),
        (3, 9, "cab".to_string()),
    ];

    for (n, k, expected) in test_cases {
        let result = Solution::get_happy_string(n, k);
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (n, k),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}
