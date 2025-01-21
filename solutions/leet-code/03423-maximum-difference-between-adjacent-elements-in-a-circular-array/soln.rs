impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let max_diff = nums.windows(2).map(|pair| (pair[1] - pair[0]).abs()).max().unwrap();
        std::cmp::max(max_diff, (nums[nums.len() - 1] - nums[0]).abs())
    }
}
