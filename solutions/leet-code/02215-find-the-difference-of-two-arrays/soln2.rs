use std::collections::BTreeSet;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut temp = BTreeSet::from_iter(nums1.clone());
        let mut temp2 = BTreeSet::from_iter(nums2.clone());
        return vec![temp.difference(&temp2).copied().collect(),temp2.difference(&temp).copied().collect()];
    }
}
