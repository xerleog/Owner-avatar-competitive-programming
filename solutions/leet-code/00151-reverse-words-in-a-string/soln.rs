impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut b :String = "".to_string();
        for i in s.split_whitespace().rev()
        {
            b = b+i+" ";
        }
        return String::from(b.trim());
    }
}
