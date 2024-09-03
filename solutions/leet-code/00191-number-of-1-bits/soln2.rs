impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut s = format!("{n:b}").to_string();
        let mut cnt:i32=0;
        for i in s.chars()
        {
            if i=='1'
            { cnt+=1;}
        }
        cnt
    }
    
}
