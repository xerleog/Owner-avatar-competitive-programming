use std::collections::BTreeSet;
impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let ans = BTreeSet::from_iter(s.chars());
        ans.len() as i32
    }
}
