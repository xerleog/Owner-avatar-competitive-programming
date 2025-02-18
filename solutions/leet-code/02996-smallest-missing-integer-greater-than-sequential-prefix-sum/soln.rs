impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        for i in 1..nums.len()
        {
            if nums[i]==nums[i-1]+1
            {   ans+=nums[i];}
            else
            {  break;}
        }
        while nums.contains(&ans)
        {   ans+=1;}
        ans
    }
}
