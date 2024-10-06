impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let mut sol = 0;
        for i in 0..nums.len()
        {
            for j in 0..nums.len()
            {
                if (nums[i]-nums[j]).abs()<=nums[i].min(nums[j])
                {   sol = sol.max(nums[i]^nums[j]);}
            }
        }
        sol
    }
}
