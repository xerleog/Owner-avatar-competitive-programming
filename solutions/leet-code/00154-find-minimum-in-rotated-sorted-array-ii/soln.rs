use std::cmp::min;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        for i in 0..nums.len()
        {
            ans = min(ans,nums[i]);
        }
        return ans;
    }
}
