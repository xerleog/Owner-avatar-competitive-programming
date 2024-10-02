impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut a,mut b) = (a,b);
        while b!=0
        {
            let car = (a&b)<<1;
            a = a^b;
            b = car;
        }
        a   
    }
}
