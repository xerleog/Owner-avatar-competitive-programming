use std::collections::{HashSet,HashMap};
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans :HashSet<(i32,i32)> = HashSet::new();
        let mut sol :HashMap<i32,i32> = HashMap::new();
        for i in &nums
        {   *sol.entry(*i).or_default()+=1; }
        for i in nums.iter()
        {
            if *i == i+k && *sol.get(i).unwrap()>1
            {   ans.insert((*i,*i));}
            if nums.contains(&(i+k)) && *i!=i+k 
            {   ans.insert((*i,i+k)); }
        }
        ans.len() as i32
    }
}
