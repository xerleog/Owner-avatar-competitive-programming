impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut ans = s.trim_matches('0');
        if ans.contains(&"0")
        {   return false;}
        else
        {   return true;}
    }
}
