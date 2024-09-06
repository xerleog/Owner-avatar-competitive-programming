impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans =vec![];
        let mut prod =1;
        for i in 0..nums.len()
        {
            prod*=nums[i];
        }
        for j in 0..nums.len()
        {
            if nums[j]==0
            {
                let mut temp = 1;
                for i in 0..nums.len()
                {
                    if i!=j
                    {   temp*=nums[i];}
                }
                ans.insert(j,temp);
            }
            else
            {   ans.insert(j,prod/nums[j]);}
        }
        ans
    }
}
