impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
       let mut ans = vec![0;(n+3) as usize];
       ans[0]=0;
       ans[1]=1;
       ans[2]=1;
       for i in 3..=(n+2) as usize
       {
            ans[i]=ans[i-1]+ans[i-2]+ans[i-3];
       }
       ans[n as usize] 
    }
}
