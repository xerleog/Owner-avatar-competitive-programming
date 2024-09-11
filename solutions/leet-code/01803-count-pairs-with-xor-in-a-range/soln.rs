impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        let mut cnt =0;
        for i in 0..nums.len()-1
        {
            for j in i+1..nums.len()
            {
                let temp = nums[i]^nums[j];
                if (temp>=low)&&(temp<=high)
                {   cnt+=1;}
            }
        }
        cnt
    }
}
