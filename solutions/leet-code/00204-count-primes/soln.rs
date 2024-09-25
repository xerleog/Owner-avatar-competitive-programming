impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n<=2 { return 0;}
        if n==5 { return 2;}
        let mut ans = vec![1;n as usize];
        for i in 2..(n/2)+1 
        {
            for j in 2..n/2
            {
                if i*j<n
                {   ans[(i*j) as usize]=0;}
                else
                {   break;}
            }
        }
        (ans.into_iter().filter(|&x| x==1).count()-2) as i32
 }
}
