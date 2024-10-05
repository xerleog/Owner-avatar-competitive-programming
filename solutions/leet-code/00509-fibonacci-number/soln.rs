impl Solution {
    pub fn fib(n: i32) -> i32 {
       let mut ans = vec![0;(n+2) as usize];
       ans[0]=0;
       ans[1]=1;
       for i in 2..=n as usize
       {
            ans[i]=ans[i-1]+ans[i-2];
       } 
       ans[n as usize]
    }
}
