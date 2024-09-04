impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mi:i32=nums.len() as i32;
        let mut ma:i32=-1;
        for i in 0..nums.len()
        {
            if nums[i]==target
            {
                mi = mi.min(i as i32);
                ma = ma.max(i as i32);
            }
        }
        if mi==nums.len() as i32
        {   return vec![-1,ma];}
        else
        {   return vec![mi,ma];}
    }
}
