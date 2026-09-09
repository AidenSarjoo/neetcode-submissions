use std::collections::HashMap; 

pub fn make_counter(word : &str) -> [i32; 26] {
    let mut counts = [0i32; 26];

    for c in word.bytes() {
        counts[(c - b'a') as usize] += 1;
    }
    return counts;
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for word in strs {
            let counter = make_counter(&word);
            groups.entry(counter).or_default().push(word);
        }
        return groups.into_values().collect();
    }
}
