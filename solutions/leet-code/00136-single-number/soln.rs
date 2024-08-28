impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in nums.iter()
        {
            ans = ans^i;
        }
        return ans;
    }
}
