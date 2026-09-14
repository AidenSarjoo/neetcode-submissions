impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new(); 
        res.extend(nums.clone());
        res.extend(nums.clone());
        res
    }
}
