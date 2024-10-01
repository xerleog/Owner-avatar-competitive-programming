impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        !s.trim_matches('0').contains(&"0")
    }
}
