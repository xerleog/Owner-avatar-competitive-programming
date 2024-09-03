impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l:usize = 0;
    let mut r:usize = numbers.len()-1;
      while l<r
      {
        let mut val = numbers[l]+numbers[r];
        if val==target
        {   return vec![(l+1) as i32,(r+1) as i32];}
        else if val<target
        { l+=1;}
        else if val>target
        { r-=1;}
      }
      return vec![0];
    }
}
