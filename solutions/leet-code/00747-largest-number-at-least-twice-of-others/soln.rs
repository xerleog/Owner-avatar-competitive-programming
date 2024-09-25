impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let ma = nums.clone().into_iter().max().unwrap();
        for i in 0..n
        {
            if !(nums[i]*2<=ma|| nums[i]==ma)
            {   return -1;}
        } 
        nums.iter().position(|&x| x==ma).unwrap() as i32
    }
}
