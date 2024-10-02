impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut ans = vec![];
        let mut cnt = 1;
        let mut known = chars[0];
        for i in 1..chars.len()
        {
            if chars[i]!=known
            {
                ans.push(known);
                if cnt>1
                {
                    for j in cnt.to_string().chars()
                    {   ans.push(j);}
                }
                known = chars[i];
                cnt=1;
            }
            else
            {   cnt+=1;}
        }
        ans.push(known);
        if cnt>1
        {
            for j in cnt.to_string().chars()
            {   ans.push(j);}
        }
        chars.clear();
        *chars = ans;
        chars.len() as i32
    }
}
