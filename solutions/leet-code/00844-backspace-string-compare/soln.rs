impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
       let mut sb = vec![]; 
       let mut tb = vec![];
       for i in s.as_bytes()
       {    if *i==35_u8 { sb.pop();}
            else { sb.push(*i);}
       } 
       for i in t.as_bytes()
       {    if *i==35_u8 { tb.pop();}
            else { tb.push(*i);}
       }
       sb==tb
    }
}
