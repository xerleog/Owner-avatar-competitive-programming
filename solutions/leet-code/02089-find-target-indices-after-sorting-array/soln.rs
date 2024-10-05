impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
       let mut ans = nums.clone();
       let mut sol = vec![];
       ans.sort();
        for i in 0..ans.len()
        {
            if ans[i]==target
            {   sol.push(i as i32);}
        }
        sol
    }
}
