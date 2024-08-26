impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut ans= [nums1,nums2].concat();
        ans.sort();
        let n= ans.len();
        if n%2==1
        {
            return ans[n/2] as f64;
        }
        else
        {
            return (ans[n/2-1]+ans[n/2] )as f64/2.0;
        }
    }
}
