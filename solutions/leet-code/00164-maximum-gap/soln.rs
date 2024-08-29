use std::cmp::max;
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut sol = nums.clone();
        if sol.len()==1
        {return 0;}
        sol.sort();
        let mut ans=0;
        for i in 1..sol.len()
        {
            ans = max(ans,sol[i]-sol[i-1]);
        }
        return ans;

    }
}
