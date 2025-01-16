impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums1.len()
        {
            for j in 0..nums2.len()
            {
                ans^=(nums1[i]^nums2[j]);                
            }
        }
        ans
    }
}
