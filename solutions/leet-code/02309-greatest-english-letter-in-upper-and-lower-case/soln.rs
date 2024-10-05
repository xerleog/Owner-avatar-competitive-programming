impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let ans = s.as_bytes().to_vec();
        let mut temp :char = ' ';
        for i in &ans
        {
            if ans.contains(&(i+32))
            {   temp = temp.max(*i as char);}
        }
        temp.to_string().trim().to_string()
    }
}
