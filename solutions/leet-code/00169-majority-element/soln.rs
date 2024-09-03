use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map:HashMap<i32,i32> = HashMap::new();
    
        for val in nums {
            *map.entry(val).or_default()+=1;
        }
        let mut ans = -1;
        let mut id = -1;
        for (i,j) in map.clone()
        {
            if(j>ans)
            {
                ans = j;
                id = i;
            }
        }
        return id as i32;
    }
}
