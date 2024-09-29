impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut cnt = 0;
        for i in words
        {
            let mut k=0;
            for j in i.chars()
            {
                k+=1;
                if !allowed.contains(j)
                {   break;}
                if k==i.len()
                {   cnt+=1;}
            }
        }
        cnt
    }
}
