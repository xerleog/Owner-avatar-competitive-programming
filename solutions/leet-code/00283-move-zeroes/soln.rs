impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i=0;
        let mut cnt =0;
        while i<nums.len()-cnt
        {
            if nums[i]==0
            {
                cnt+=1;
                nums.remove(i);
                i-=1;
                nums.push(0);
            }
            i+=1;
        }

    }
}
