impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let pad = nums[0].len();
        for i in 0..=2_u32.pow(pad as u32)
        {
            if !nums.contains(&format!("{i:0m$b}",m=pad))
            {   return format!("{i:0m$b}",m=pad);}
        }
        '0'.to_string()
    }
}

