use std::collections::HashMap; 

impl Solution {

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {return false; }

        let mut sm = HashMap::new();
        let mut tm = HashMap::new();

        for c in s.chars() {
            *sm.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *tm.entry(c).or_insert(0) += 1;
        }

        return sm == tm
    }
}
