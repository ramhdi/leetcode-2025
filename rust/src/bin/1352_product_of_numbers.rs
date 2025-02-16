// https://leetcode.com/problems/product-of-the-last-k-numbers/
// 14/02/2025

#[derive(Debug, Clone)]
struct ProductOfNumbers {
    products: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers {
            products: Vec::new(),
        }
    }

    fn add(&mut self, num: i32) {
        match num {
            0 => self.products.clear(),
            _ => match self.products.last() {
                Some(p) => self.products.push(p * num),
                None => self.products.push(num),
            },
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let (n, k) = (self.products.len(), k as usize);
        if n == 0 || k > n {
            return 0;
        }

        if n == k {
            return self.products[n - 1];
        }

        return self.products[n - 1] / self.products[n - k - 1];
    }
}

fn main() {
    let test_cases = vec![(
        vec![
            "ProductOfNumbers",
            "add",
            "add",
            "add",
            "add",
            "add",
            "getProduct",
            "getProduct",
            "getProduct",
            "add",
            "getProduct",
        ],
        vec![
            vec![],
            vec![3],
            vec![0],
            vec![2],
            vec![5],
            vec![4],
            vec![2],
            vec![3],
            vec![4],
            vec![8],
            vec![2],
        ],
        vec![
            None,
            None,
            None,
            None,
            None,
            None,
            Some(20),
            Some(40),
            Some(0),
            None,
            Some(32),
        ],
    )];

    for (instruction, param, expected) in test_cases {
        let result: Vec<Option<i32>> = instruction
            .iter()
            .zip(param.iter())
            .scan(ProductOfNumbers::new(), |pn, (i, p)| match i {
                &"add" => {
                    pn.add(p[0]);
                    Some(None)
                }
                &"getProduct" => Some(Some(pn.get_product(p[0]))),
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
