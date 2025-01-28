use std::collections::HashMap;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let mut s_hash: HashMap<char, i32> = HashMap::new();
        let mut goal_hash: HashMap<char, i32> = HashMap::new();

        for i in s.chars() {
            *s_hash.entry(i).or_insert(0) += 1;
        }
        for i in goal.chars() {
            *goal_hash.entry(i).or_insert(0) += 1;
        }
        println!("{:?}", s_hash);
        println!("{:?}", goal_hash);

        if s_hash == goal_hash {
            let mut count = 0;
            for (a, b) in s.chars().zip(goal.chars()) {
                if a != b {
                    count += 1;
                }
            }
            if count > 2 {
                return false;
            }

            if s_hash.values().any(|x| x > &1) || count == 2 {
                return true;
            }
        }
        return false;
    }
}
