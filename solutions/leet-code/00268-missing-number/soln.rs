impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n :i32 = (nums.len()+1) as i32;
        for i in 0..=n
        {
            if !nums.contains(&i)
            {   return i;}
        }
        return n;
    }
}
