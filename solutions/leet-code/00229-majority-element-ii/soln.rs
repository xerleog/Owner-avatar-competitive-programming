use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut ans:HashMap<i32,i32> =HashMap::new();
        let mut sol = vec![];
        let k :i32 =nums.len() as i32/3;
        for i in nums
        {
            *ans.entry(i).or_default()+=1;
        }
        for (val,id) in ans
        {
            if id>k
            {   sol.push(val);}
        }
        sol
    }
}
