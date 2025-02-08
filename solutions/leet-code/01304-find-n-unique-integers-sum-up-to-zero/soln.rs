impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
      let mut ans = vec![];
      for i in 1..=n/2
      {
        ans.push(i);
        ans.push(i*-1);
      }
      if n%2==1
      { ans.push(0);}
      ans

    }
}
