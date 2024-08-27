impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len()< needle.len() { return -1;}
        for ind in 0..haystack.len()-needle.len()+1
        {
            let mut temp:String = haystack[ind..ind+needle.len()].to_string();
            if temp == needle
            { return ind as i32;}
        }
        return -1;
    }
}
