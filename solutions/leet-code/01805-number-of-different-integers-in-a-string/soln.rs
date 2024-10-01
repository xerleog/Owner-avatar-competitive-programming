use std::collections::BTreeSet;
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut sol = BTreeSet::new();
        let mut temp = String::new();
        for i in word.chars()
        {
            if i.is_alphabetic() && temp.len()>0
            {   sol.insert(temp.trim_start_matches('0').to_string());
                temp.clear();
            }
            else if i.is_ascii_digit()
            {   temp.push(i);}
        }
        if temp.len()>0 { sol.insert(temp.trim_start_matches('0').to_string());}
        sol.len() as i32
    }
}
