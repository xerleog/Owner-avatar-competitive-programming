impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
      for i in 0..numbers.len()-1
      {
        for j in i..numbers.len()
        {
            if (i!=j)&&(numbers[i]+numbers[j]==target)
            { return vec![(i+1) as i32,(j+1) as i32];}
        }
      }
      return vec![0];  
    }
}
