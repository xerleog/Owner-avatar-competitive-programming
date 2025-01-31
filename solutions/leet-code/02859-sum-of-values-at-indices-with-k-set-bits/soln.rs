impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        (0..nums.len()).filter(|x| x.count_ones() as i32==k).fold(0,|a,c| a+nums[c as usize])
    }
}
