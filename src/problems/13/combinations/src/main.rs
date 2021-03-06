// Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.

// Example:

// Input: n = 4, k = 2
// Output:
// [
//   [2,4],
//   [3,4],
//   [2,3],
//   [1,2],
//   [1,3],
//   [1,4],
// ]

fn main() {
    let result = Solution::combine(4, 2);
    let expected = vec![
        vec![2, 4],
        vec![3, 4],
        vec![2, 3],
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
    ];
    println!("expected {:?}", expected);
    assert_eq!(expected.len(), result.len());
    for value in expected {
        assert_eq!(true, result.contains(&value));
    }

    Solution::combine(13, 10);
}

struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut all_solutions: Vec<Vec<i32>> = vec![];
        Self::combine_help(n, k, 1, &mut Vec::new(), &mut all_solutions);
        all_solutions
    }

    fn combine_help(
        n: i32,
        k: i32,
        index: i32,
        one_solution: &mut Vec<i32>,
        all_solutions: &mut Vec<Vec<i32>>,
    ) {
        // println!("--{:?} {:?}", k, index);
        if k == 0 {
            // println!("==>{:?}", one_solution);
            all_solutions.push(one_solution.to_vec());
            return;
        }

        for i in index..=n - k + 1 {
            one_solution.push(i);
            Self::combine_help(n, k - 1, i + 1, one_solution, all_solutions);
            one_solution.pop();
        }
    }
}
