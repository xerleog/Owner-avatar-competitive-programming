impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for i in nums2
        {
            if nums1.contains(&i) && !ans.contains(&i)
            {   ans.push(i);}
        }
        ans
    }
}
