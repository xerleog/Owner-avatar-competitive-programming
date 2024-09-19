impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut ans = start;
        for i in 1..n
        {
            ans=ans^(start+(2*i));
        }
        ans
    }
}
