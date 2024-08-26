impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x<0 {return false;}
        let sep = x.to_string();
        let res = sep.chars()
                        .rev()
                        .collect::<String>();
        if sep==res
        {   return true;}
        else
        {   return false;}
    }
}
