use std::collections::HashSet;
impl Solution {
    pub fn is_number(s: String) -> bool {
        let b = ["inf","-inf","+inf","nan","infinity","-infinity","+infinity"];
        for i in b
        {
            if i ==s.to_lowercase()
            {    return false;}
        }
        return s.parse::<f64>().is_ok();
    }
}
