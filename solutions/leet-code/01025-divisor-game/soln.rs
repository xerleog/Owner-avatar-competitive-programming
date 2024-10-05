impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        let mut cnt = 0;
        let mut n = n;
        while n>1
        {
            for i in 1..n
            {
                if n%i==0
                {
                    cnt+=1;
                    n-=i;
                    break;
                }
            }
        }
        cnt%2==1
    }
}
