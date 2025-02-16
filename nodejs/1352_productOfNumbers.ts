// https://leetcode.com/problems/product-of-the-last-k-numbers/
// 14/02/2025

class ProductOfNumbers {
    products: number[];

    constructor() {
        this.products = [];
    }

    add(num: number): void {
        if (num === 0) {
            this.products.length = 0; // clear the array
        } else if (this.products.length === 0) {
            this.products.push(num);
        } else {
            const p = this.products[this.products.length - 1];
            this.products.push(num * p);
        }
    }

    getProduct(k: number): number {
        const n = this.products.length;
        if (n === 0 || k > n) {
            return 0;
        }

        if (n === k) {
            return this.products[n - 1];
        }

        return this.products[n - 1] / this.products[n - k - 1];
    }
}


// (() => {
//     const testCases: [number, number[][], number[]][] = [
//         [
//             4,
//             [[1, 4], [2, 5], [1, 3], [3, 4]],
//             [1, 2, 2, 3]
//         ],
//         [
//             4,
//             [[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]],
//             [1, 2, 2, 3, 4]
//         ],
//     ];

//     testCases.forEach(([limit, queries, expected], i) => {
//         console.log(`Test Case #${i}:`);
//         const result = queryResults(limit, queries);
//         console.log(`Input: ${[limit, queries]}, Expected: ${expected}, Got: ${result}`);
//         _.isEqual(result, expected) ? console.log("PASS") : console.log("FAIL");
//     })
// })();
