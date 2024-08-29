use std::cmp::min;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        for i in nums
        {
            ans = min(ans,i);
        }
        return ans;
    }
}
