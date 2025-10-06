use std::{cmp, collections::HashSet};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut max: usize = 0;

        for i in 0..chars.len() {
            let mut seen: HashSet<char> = HashSet::new();
            for j in i..chars.len() {
                if seen.contains(&chars[j]) {
                    break;
                }
                seen.insert(chars[j]);
                max = cmp::max(max, seen.len());
            }
        }

        max as i32
    }
}
