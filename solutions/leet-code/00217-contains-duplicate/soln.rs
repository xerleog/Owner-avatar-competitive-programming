use std::collections::HashMap;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map :HashMap<i32,usize>=HashMap::new();
        for i in nums
        {
            *map.entry(i).or_default()+=1;
            if map[&i]>1
            {   return true;}
        }
        return false;
    }
}
