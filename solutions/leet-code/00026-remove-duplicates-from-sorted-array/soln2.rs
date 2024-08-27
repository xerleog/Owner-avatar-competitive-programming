use std::collections::HashSet;
use std::mem::replace;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut set= HashSet::new();
        let mut ans= Vec::new();
        for i in nums.iter()
        {
            if !set.contains(i)
            {
                set.insert(i);
                ans.push(*i);
            }
        }
        replace(nums,ans);
        return nums.len() as i32;
    }
}
