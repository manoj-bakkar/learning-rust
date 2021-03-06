// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.

// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

// Example:

// Input: "23"
// Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].

fn main() {
    Solution::letter_combinations(String::from("23456789"));
}

struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut candidates = Solution::get_candidates(&digits);
        if candidates.len() == 0 {
            return vec![];
        }
        let current_candidates = candidates.remove(0);
        let first_iteration_result =
            Solution::append_to_string(&current_candidates, String::from(""));

        let final_result = Solution::helper(&mut candidates, first_iteration_result);
        println!("final_result {:?}", final_result);
        return final_result;
    }

    pub fn helper(
        candidates: &mut Vec<Vec<&'static str>>,
        combinations: Vec<String>,
    ) -> Vec<String> {
        // println!("candidates {:?}, result {:?}", candidates, combinations);
        if candidates.len() == 0 {
            return combinations;
        }

        let current_candidates = candidates.remove(0);
        let mut iteration_result = vec![];

        for combination in combinations {
            let result_append = Solution::append_to_string(&current_candidates, combination);
            for i in result_append {
                iteration_result.push(i);
            }
        }

        let final_result = Solution::helper(candidates, iteration_result);
        return final_result;
    }

    pub fn get_candidates(digits: &String) -> Vec<Vec<&'static str>> {
        let mut candidates = vec![];
        for c in digits.chars() {
            let letters = Solution::get_letters(c.to_string());
            candidates.push(letters);
        }
        return candidates;
    }

    pub fn get_letters(digit: String) -> Vec<&'static str> {
        if String::from("2").eq(&digit) {
            return vec!["a", "b", "c"];
        } else if String::from("3").eq(&digit) {
            return vec!["d", "e", "f"];
        } else if String::from("4").eq(&digit) {
            return vec!["g", "h", "i"];
        } else if String::from("5").eq(&digit) {
            return vec!["j", "k", "l"];
        } else if String::from("6").eq(&digit) {
            return vec!["m", "n", "o"];
        } else if String::from("7").eq(&digit) {
            return vec!["p", "q", "r", "s"];
        } else if String::from("8").eq(&digit) {
            return vec!["t", "u", "v"];
        } else if String::from("9").eq(&digit) {
            return vec!["w", "x", "y", "z"];
        }
        return vec![];
    }

    fn append_to_string(vec: &Vec<&'static str>, string: String) -> Vec<String> {
        let mut result = vec![];

        for i in vec {
            result.push(format!("{}{}", string, i));
        }
        return result;
    }
}

// fn _main() {
//     let one: Vec<&'static str> = vec!["a", "b", "c"];
//     let two: Vec<&'static str> = vec!["d", "e", "f"];
//     let three: Vec<&'static str> = vec!["g", "h", "i"];
//     // let four = vec!["d", "e", "f"];
//     // let five = vec!["d", "e", "f"];
//     // let six = vec!["d", "e", "f"];
//     // let seven = vec!["d", "e", "f"];
//     // let eight = vec!["d", "e", "f"];
//     // let nine = vec!["d", "e", "f"];

//     let result_one = append_to_string(&one, String::from(""));
//     println!("result_one {:?}", result_one);

//     let mut result_two = vec![];
//     for result in result_one {
//         let result_append = append_to_string(&two, result);
//         for i in result_append {
//             result_two.push(i);
//         }
//     }
//     println!("result_two {:?}", result_two);

//     let mut result_three = vec![];
//     for result in result_two {
//         let result_append = append_to_string(&three, result);
//         for i in result_append {
//             result_three.push(i);
//         }
//     }
//     println!("result_three {:?}", result_three);
// }

// fn append_to_string(vec: &Vec<&'static str>, string: String) -> Vec<String> {
//     let mut result = vec![];

//     for i in vec {
//         result.push(format!("{}{}", string, i));
//     }
//     return result;
// }
