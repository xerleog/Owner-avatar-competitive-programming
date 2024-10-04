impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
       if nums.len()==1 {return nums[0];}
       for i in 1..nums.len()-1
       {
            if nums[i]!=nums[i-1]&&nums[i]!=nums[i+1]
            {   return nums[i];}
       }
       if nums[nums.len()-1]==nums[nums.len()-2] { return nums[0];}
       nums[nums.len()-1]
    }
}
