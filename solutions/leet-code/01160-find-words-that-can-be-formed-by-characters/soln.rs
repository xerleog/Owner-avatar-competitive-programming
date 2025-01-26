impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut ans = vec![0;26];
        let mut val =0;
        chars.chars().for_each(|x| ans[(x as u8-'a' as u8)as usize]+=1);
        for i in words
        {
            let mut sol = vec![0;26];
            i.chars().for_each(|x| sol[(x as u8-'a' as u8)as usize]+=1);
            if(ans.clone().into_iter().zip(sol.into_iter()).all(|(a,b)| b<=a))
            {   val+=i.len() as i32;}
        }
        val
    }
}
