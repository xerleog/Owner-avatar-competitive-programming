impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        for i in 0..n-1
        {
            for j in i+1..n
            {
                if nums[i]>nums[j]
                { nums.swap(i,j);}
            }
        }
    }
}
