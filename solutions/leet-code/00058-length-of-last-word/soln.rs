impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
       let mut ans = Vec::new();
       for i in s.split_whitespace()
       {
            ans.push(i);
       }
       return ans[ans.len()-1].len() as i32;

    }
}
