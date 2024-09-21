use std::collections::HashSet;
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let ans :HashSet<i32>= HashSet::from_iter(nums.into_iter());
        let mut sol = ans.iter().collect::<Vec<_>>();
        sol.sort();
        if sol.len()<3
        {   return *sol[sol.len()-1];}
        else
        {   return *sol[sol.len()-3];}
    }
}
