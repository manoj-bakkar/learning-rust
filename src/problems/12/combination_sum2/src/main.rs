// Given a collection of candidate numbers (candidates) and a target number (target),
// find all unique combinations in candidates where the candidate numbers sums to target.

// Each number in candidates may only be used once in the combination.

// Note:

// All numbers (including target) will be positive integers.
// The solution set must not contain duplicate combinations.

// Example 1:
// Input: candidates = [10,1,2,7,6,1,5], target = 8,
// A solution set is:
// [
//   [1, 7],
//   [1, 2, 5],
//   [2, 6],
//   [1, 1, 6]
// ]

// Example 2:
// Input: candidates = [2,5,2,1,2], target = 5,
// A solution set is:
// [
//   [1,2,2],
//   [5]
// ]

fn main() {
    let result = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
    let expected = vec![vec![1, 2, 2]];
    assert_eq!(expected.len(), result.len());
    for value in expected {
        assert_eq!(true, result.contains(&value));
    }
}

struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let result: Vec<Vec<i32>> = vec![];
        println!("candidates {:?}, target {:?}", candidates, target);
        let candidates_clone = candidates.clone();
        Solution::recursion(&candidates_clone, target, &result);
        return result;
    }

    pub fn recursion(candidates: &Vec<i32>, target: i32, result: &Vec<Vec<i32>>) {
        println!("candidates {:?}, target {:?}", candidates, target);
    }
}
