use std::collections::HashMap;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut ans :HashMap<i32,i32> = HashMap::new();
        for i in nums
        {
            *ans.entry(i).or_default()+=1;
            if ans[&i]==2
            {   return i;}
        }
        return 0;
    }
}
