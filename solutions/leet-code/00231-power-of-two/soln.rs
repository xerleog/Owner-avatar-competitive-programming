impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n<=0 {return false;}
        let mut n = n;
        while (n>1)
        {
            if n%2!=0
            {   return false;}
            n/=2;
        }
        return true;
    }
}
