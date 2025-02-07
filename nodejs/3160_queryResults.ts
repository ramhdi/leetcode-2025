// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/
// 07/02/2025

import _ from 'lodash';

function queryResults(limit: number, queries: number[][]): number[] {
    let ballColors = new Map<number, number>();  // Maps ball -> color
    let colorCount = new Map<number, number>();  // Maps color -> count
    let result: number[] = new Array(queries.length);
    let distinctColors = 0;

    for (let i = 0; i < queries.length; i++) {
        let [ball, newColor] = queries[i];

        if (ballColors.has(ball)) {
            let oldColor = ballColors.get(ball)!;
            if (oldColor === newColor) {
                result[i] = distinctColors;
                continue;
            }

            // Decrease count of old color
            if (colorCount.get(oldColor) === 1) {
                colorCount.delete(oldColor);
                distinctColors--;
            } else {
                colorCount.set(oldColor, colorCount.get(oldColor)! - 1);
            }
        }

        // Assign new color to ball
        ballColors.set(ball, newColor);

        // Increase count of new color
        if (!colorCount.has(newColor)) {
            colorCount.set(newColor, 1);
            distinctColors++;
        } else {
            colorCount.set(newColor, colorCount.get(newColor)! + 1);
        }

        result[i] = distinctColors;
    }

    return result;
}


(() => {
    const testCases: [number, number[][], number[]][] = [
        [
            4,
            [[1, 4], [2, 5], [1, 3], [3, 4]],
            [1, 2, 2, 3]
        ],
        [
            4,
            [[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]],
            [1, 2, 2, 3, 4]
        ],
    ];

    testCases.forEach(([limit, queries, expected], i) => {
        console.log(`Test Case #${i}:`);
        const result = queryResults(limit, queries);
        console.log(`Input: ${[limit, queries]}, Expected: ${expected}, Got: ${result}`);
        _.isEqual(result, expected) ? console.log("PASS") : console.log("FAIL");
    })
})();
