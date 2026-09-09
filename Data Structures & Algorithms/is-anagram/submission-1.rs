use std::collections::HashMap; 

impl Solution {

    pub fn is_anagram(s: String, t: String) -> bool {
        let mut sm = HashMap::new();
        let mut tm = HashMap::new();

        for c in s.chars() {
            *sm.entry(c).or_insert(1) += 1;
        }
        for c in t.chars() {
            *tm.entry(c).or_insert(1) += 1;
        }

        return sm == tm
    }
}
