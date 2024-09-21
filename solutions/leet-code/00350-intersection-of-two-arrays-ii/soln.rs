impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut nums1 = nums1.clone();
        let mut nums2 = nums2.clone();
        nums1.sort();
        nums2.sort();
        let mut i:usize =0;
        let mut j:usize = 0;
        while i<nums1.len() && j<nums2.len()
        {
            if nums1[i]==nums2[j]
            {
                ans.push(nums1[i]);
                i+=1;
                j+=1;
            }
            else if nums1[i]>nums2[j]
            {   j+=1;}
            else
            {   i+=1;}
        }
        ans
    }
}
