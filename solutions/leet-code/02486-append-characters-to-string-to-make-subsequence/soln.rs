impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let (sb,tb) = (s.as_bytes(),t.as_bytes());
        let (mut i,mut j) =(0,0);
        while i<s.len()&&j<t.len()
        {
            if sb[i]==tb[j]
            {   j+=1;}
            i+=1;
        }
        (t.len()-j) as i32

    }
}
