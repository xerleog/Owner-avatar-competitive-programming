use std::collections::HashSet;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = HashSet::new();
        for i in nums.iter()
        {
            if ans.contains(i)
            {   ans.remove(i);}
            else
            {   ans.insert(*i);}
        }
        ans.into_iter().collect::<Vec<i32>>()
    }
}
