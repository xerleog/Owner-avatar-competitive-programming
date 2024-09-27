use std::collections::BTreeMap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans :BTreeMap<i32,i32> = BTreeMap::new();
        let mut k = k;
        for i in nums
        {   *ans.entry(i).or_default()+=1;}
        for (val,id) in ans.iter().rev()
        {
            if k-id<=0
            {
                return *val;
                break;
            }
            k-=id;
        }
        0
    }
}
