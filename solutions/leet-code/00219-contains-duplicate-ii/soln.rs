impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        for i in 0..nums.len()
        {
            for j in i+1..=i+k as usize
            {
                if j<nums.len() &&nums[i]==nums[j]
                {   return true;}
            }
        }
        return false;
    }
}
