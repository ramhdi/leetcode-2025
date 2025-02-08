// https://leetcode.com/problems/design-a-number-container-system/
// 08/02/2025

use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    index_to_num: HashMap<i32, i32>,
    num_to_indices: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            index_to_num: HashMap::new(),
            num_to_indices: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old_num) = self.index_to_num.get(&index) {
            if let Some(indices) = self.num_to_indices.get_mut(&old_num) {
                indices.remove(&index);
                if indices.is_empty() {
                    self.num_to_indices.remove(&old_num);
                }
            }
        }

        self.index_to_num.insert(index, number);

        self.num_to_indices
            .entry(number)
            .or_insert_with(BTreeSet::new)
            .insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        if let Some(indices) = self.num_to_indices.get(&number) {
            if let Some(&smallest_index) = indices.iter().next() {
                return smallest_index;
            }
        }
        -1
    }
}

fn main() {
    let test_cases = vec![(
        vec![
            "NumberContainers",
            "find",
            "change",
            "change",
            "change",
            "change",
            "find",
            "change",
            "find",
        ],
        vec![
            vec![],
            vec![10],
            vec![2, 10],
            vec![1, 10],
            vec![3, 10],
            vec![5, 10],
            vec![10],
            vec![1, 20],
            vec![10],
        ],
        vec![
            None,
            Some(-1),
            None,
            None,
            None,
            None,
            Some(1),
            None,
            Some(2),
        ],
    )];

    for (instruction, param, expected) in test_cases {
        let result: Vec<Option<i32>> = instruction
            .iter()
            .zip(param.iter())
            .scan(NumberContainers::new(), |nc, (i, p)| match i {
                &"find" => Some(Some(nc.find(p[0]))),
                &"change" => {
                    nc.change(p[0], p[1]);
                    Some(None)
                }
                _ => Some(None),
            })
            .collect();
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (instruction, param),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}
