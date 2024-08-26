impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut ans :Vec<i32> = vec![];
        for i in nums1
        {   ans.push(i);}
        for i in nums2
        {   ans.push(i);}
        ans.sort();
        let n = ans.len();
        if n%2==1
        {
            return ans[n/2] as f64;
        }
        else
        {   let  val:f64 = ((ans[n/2-1]+ans[(n/2)])as f64/2.0) as f64;
            return val;
        }
    }
}
