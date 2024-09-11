impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        let mut i=n;
        while i<=n*2
        {
            if i%2==0
            {   return i;}
            else
            {   i+=n;}
        }
        return n*2;
    }
}
