impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;nums.len()];
        let mut pos=0;
        let mut neg = 1;
        for i in nums
        {
            if i>=0
            {
                ans[pos]=i;
                pos+=2;
            }
            else
            {
                ans[neg]=i;
                neg+=2;
            }
        }
        ans
    }
}
