impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut mul=1;
        let mut add =0;
        let mut k=n;
        while k>0
        {
            let temp =k%10;
            mul*=temp;
            add+=temp;
            k/=10;
        }
        mul-add
    }
}
