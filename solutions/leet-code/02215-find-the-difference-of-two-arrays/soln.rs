use std::collections::BTreeSet;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut temp = BTreeSet::from_iter(nums1.clone());
        let mut temp2 = BTreeSet::from_iter(nums2.clone());
        for i in &nums1
        {
            if temp.contains(i)&&temp2.contains(i)
            {
                temp.remove(i);
                temp2.remove(i);
            }
        }
        return vec![Vec::from_iter(temp),Vec::from_iter(temp2)];
    }
}
