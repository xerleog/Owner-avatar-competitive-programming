impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
       let mut ans = right;
       for i in left..right
       {    ans&=i;
            if ans==0
            {   return 0;}
       }
       ans 
    }
}
