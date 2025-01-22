use std::collections::BTreeMap;
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans :BTreeMap<i32,usize>= BTreeMap::new();
        nums.iter().for_each(|x| { {x.into_iter().for_each(|&y| {*ans.entry(y).or_default()+=1;})};});
        ans.into_iter().filter(|w| w.1==nums.len()).map(|x| x.0).collect::<Vec<_>>()
    }
}
