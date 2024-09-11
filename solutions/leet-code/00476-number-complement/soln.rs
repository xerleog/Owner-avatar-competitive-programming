impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let temp = format!("{num:b}");
        let mut ans = "".to_string();
        for i in temp.chars() 
        {
            if i=='0' { ans+="1";}
            else {  ans+="0";}
        }
        i32::from_str_radix(ans.as_str(),2).unwrap()
    }
}
