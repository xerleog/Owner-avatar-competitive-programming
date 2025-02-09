use std::collections::HashMap;
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len() as i64;
        let mut counts: HashMap<i32, i64> = HashMap::new();
        for (i, v) in nums.into_iter().enumerate() {
            *counts.entry(v-i as i32).or_default() += 1;
        }
        n*(n-1)/2 - counts.values().map(|&c| c*(c-1)/2).sum::<i64>()
    }
}
