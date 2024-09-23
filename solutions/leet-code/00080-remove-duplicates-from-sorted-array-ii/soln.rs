use std::collections::HashMap;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut ans :HashMap<i32,i32> =HashMap::new();
        let mut i:usize=0;
        while i<nums.len()
        {
            *ans.entry(nums[i]).or_default()+=1;
            if ans[&nums[i]]>2
            {
                *ans.get_mut(&nums[i]).unwrap()-=1;
                nums.remove(i);
                i-=1;
            }
            i+=1;
        }
        nums.len() as i32
    }
}
