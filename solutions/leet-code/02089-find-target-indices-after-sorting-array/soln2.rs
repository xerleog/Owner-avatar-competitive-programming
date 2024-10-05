impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort();
        (0..nums.len()).filter_map(|i| (nums[i]==target).then_some(i as i32)).collect()
    }
}
