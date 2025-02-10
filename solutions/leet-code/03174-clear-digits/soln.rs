impl Solution {
    pub fn clear_digits(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let mut ans = vec![];
        for i in 0..s.len()
        {
            if s[i].is_alphabetic()
            {   ans.push(s[i]);}
            else
            {   ans.pop();}
        }
        ans.iter().collect::<String>()
    }
}
