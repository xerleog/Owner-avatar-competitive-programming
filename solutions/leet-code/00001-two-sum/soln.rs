impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        let mut ans = Vec::<i32>::new();
        'outer: for i in 0..n{
            for j in 0..n{ 
                if (i!=j) && (nums[i]+nums[j]==target)
                {
                    ans.push(i as i32);
                    ans.push(j as i32);
                    break 'outer;
                }
            }
        }
        return ans;
    }
}
