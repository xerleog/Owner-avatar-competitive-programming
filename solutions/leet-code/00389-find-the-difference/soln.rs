use std::collections::HashMap;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut ans :HashMap<char,i32> =HashMap::new();
        for i in t.chars()
        {
            *ans.entry(i).or_default()+=1;
        }
        for j in s.chars()
        {
            ans.insert(j,ans[&j]-1);
        }
        for (val,i) in ans
        {
            if i!=0
            {   return val;}
        }
        return ' ';
    }
}
