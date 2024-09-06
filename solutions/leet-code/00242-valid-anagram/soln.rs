impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut v1 :Vec<char> = s.chars().collect();
        let mut v2 :Vec<char> = t.chars().collect();
        v1.sort();
        v2.sort();
        let a = v1.into_iter().collect::<String>();
        let b = v2.into_iter().collect::<String>();
        if a == b
        {   return true;}
        else
        {   return false;}
    }
}
