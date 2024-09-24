use std::cmp::min;
impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut a = num1;
        let mut b = num2;
        let mut c = num3;
        let mut d = vec![];
        while a>0||b>0||c>0
        {
            d.push(min(a%10,min(b%10,c%10)));
            a/=10;
            b/=10;
            c/=10;
        }
        let mut ans=0;
        for i in d.iter().rev()
        {
            ans*=10;
            ans+=i;
        }
        ans
    }
}
