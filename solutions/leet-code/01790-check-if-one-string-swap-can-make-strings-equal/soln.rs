impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let ans = s1.chars().zip(s2.chars()).
            filter(|(x,y)|x!=y).collect::<Vec<_>>();
        ans.len() == 0 || (ans.len() == 2 && ans[0].1 == ans[1].0 && ans[0].0 == ans[1].1)

    }
}
