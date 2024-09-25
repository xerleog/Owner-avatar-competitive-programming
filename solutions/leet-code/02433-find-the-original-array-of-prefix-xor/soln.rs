impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut n = pref.len();
        if n<=1 { return pref;}
        if n==2 {  return vec![pref[0],pref[0]^pref[1]];}
        let mut ans = vec![0;n];
        n-=1;
        while n>0
        {
            ans[n]= pref[n]^pref[n-1];
            n-=1;
        }
        ans[0]=pref[0];
        ans
    }
}
