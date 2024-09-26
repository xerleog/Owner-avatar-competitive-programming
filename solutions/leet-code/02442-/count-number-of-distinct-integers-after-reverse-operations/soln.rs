use std::collections::HashSet;
impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut ans =HashSet::new();
        for i in nums
        {
            ans.insert(i);
            let mut n = i;
            let mut temp =0;
            while(n>0)
            {
                temp*=10;
                temp+=n%10;
                n/=10;
            }
            ans.insert(temp);
            
        }
        ans.len() as i32
    }
}
