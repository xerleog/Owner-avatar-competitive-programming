use std::collections::HashMap;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans :HashMap<i32,i32> = HashMap::new();
        for i in 0..nums.len()
        {
            if ans.contains_key(&nums[i])
            {   ans.insert(nums[i],ans.get(&nums[i]).unwrap()+1);}
            else
            {   ans.insert(nums[i],1);}
        }
        for (c,d) in ans.iter()
        {
            if *d==1
            { return *c as i32;}
        }
        return 0;
    }
}
