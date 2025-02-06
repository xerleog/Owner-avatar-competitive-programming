impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ans =  s.split_whitespace().collect::<Vec<_>>();
        ans.into_iter().map(|x| x.chars().rev().collect::<String>()).collect::<Vec<_>>().join(" ")
    }
}
