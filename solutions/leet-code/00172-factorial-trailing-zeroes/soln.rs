impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut cnt = 5;
        let mut ans=0;
        while  cnt<=n
        {
            ans+=(n/cnt);
            cnt*=5;
        }
        ans
    }
}
