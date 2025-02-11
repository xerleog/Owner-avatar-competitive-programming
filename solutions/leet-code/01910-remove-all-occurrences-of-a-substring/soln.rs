impl Solution {
    pub fn remove_occurrences(mut s: String, part: String) -> String {
        let mut ans = String::new();
        for i in s.chars()
        {
            ans.push(i);
            if ans.ends_with(&part)
            {
                (0..part.len()).for_each(|_| {ans.pop();});
            }
        }
        ans
    }
}
