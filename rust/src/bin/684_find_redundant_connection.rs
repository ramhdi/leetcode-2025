// https://leetcode.com/problems/redundant-connection/
// 29/01/2025

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n + 1];
        let rank = vec![0; n + 1];

        for i in 0..=n {
            parent[i] = i;
        }

        DisjointSet { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (root_x, root_y) = (self.find(x), self.find(y));
        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        true
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![-1, -1];
        let n = edges
            .iter()
            .map(|e| std::cmp::max(e[0], e[1]))
            .max()
            .unwrap() as usize;

        let mut set = DisjointSet::new(n);
        for e in edges {
            if !set.union(e[0] as usize, e[1] as usize) {
                result = e;
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
        [[1, 2], [1, 3], [2, 3]].map(|e| e.to_vec()).to_vec(),
        [2, 3].to_vec(),
    )]
    #[case(
        [[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]
            .map(|e| e.to_vec())
            .to_vec(),
        [1, 4].to_vec(),
    )]
    fn test(#[case] edges: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let result = Solution::find_redundant_connection(edges);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            [[1, 2], [1, 3], [2, 3]].map(|e| e.to_vec()).to_vec(),
            [2, 3].to_vec(),
        ),
        (
            [[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]
                .map(|e| e.to_vec())
                .to_vec(),
            [1, 4].to_vec(),
        ),
    ];

    for (edges, expected) in test_cases {
        let result = Solution::find_redundant_connection(edges.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            edges, expected, result
        );
        assert_eq!(result, expected);
    }
}
