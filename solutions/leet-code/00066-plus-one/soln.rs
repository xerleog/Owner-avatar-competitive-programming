impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
       let n = digits.len();
       let mut ans = vec![0;n];
       ans[n-1]=(digits[n-1]+1)%10;
       let mut car =(digits[n-1]+1)/10;
       for i in (0..n-1).rev()
       {
            ans[i]=(digits[i]+car)%10;
            car = (digits[i]+car)/10
       }
       if car>0
       {ans.insert(0,car);}
       ans

    }
}
