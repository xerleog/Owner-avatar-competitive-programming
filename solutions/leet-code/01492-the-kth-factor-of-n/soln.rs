impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut cnt=0;
        for i in 1..n+1
        {
            if n%i==0
            {   cnt+=1;}
            if cnt ==k
            {   return i;}
        }
        return -1;
    }
}
