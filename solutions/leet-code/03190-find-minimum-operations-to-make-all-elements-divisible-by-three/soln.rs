impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut cnt=0;
        for i in nums
        {
            if !(i%3==0)
            {   cnt+=1;}
        }
        cnt
    }
}
