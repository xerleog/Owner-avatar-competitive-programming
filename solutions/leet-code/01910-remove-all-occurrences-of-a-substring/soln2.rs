impl Solution {
    pub fn remove_occurrences(mut s: String, part: String) -> String {
        let mut ans = s;
        while let Some(a)=ans.find(&part)
        {   ans.replace_range(a..a+part.len(),"");}
        ans
    }
}
