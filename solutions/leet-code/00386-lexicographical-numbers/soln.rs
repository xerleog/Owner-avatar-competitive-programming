impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = Vec::from_iter(1..=n);
        ans.sort_by(|a,b| a.to_string().cmp(&b.to_string()));
        ans   
    }
}
