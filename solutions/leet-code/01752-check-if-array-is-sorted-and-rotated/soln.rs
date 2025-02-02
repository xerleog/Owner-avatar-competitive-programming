impl Solution {
    pub fn check(nums: Vec<i32>) -> bool
    {
        let mut count = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                count += 1;
            }
            if count > 1 {
                return false;
            }
        }
        if nums[nums.len() - 1] > nums[0] {
            count += 1;
        }
        count <= 1
        }
    }
