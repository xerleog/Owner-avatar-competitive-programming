impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut t :Vec<char> = vec![];
        for i in s.chars()
        {
            if i.is_alphanumeric()
            {
                t.push(i.to_ascii_lowercase());
            }
        }
        let a = t.iter().collect::<String>();
        let b = t.iter().rev().collect::<String>();
        if a==b
        {   return true;}
        else
        {   return false;}
    }
}
