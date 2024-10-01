impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut cl = 1;
        let mut ml = 1;
        let mut ma = nums[0];
        for i in nums.into_iter().skip(1)
        {
            if i>ma
            {
                ma = i;
                cl = 1;
                ml = 1;
            }
            else if i==ma
            {
                cl+=1;
                ml = ml.max(cl);
            }
            else
            {
                cl = 0;
            }
        }
        ml
    }

}
