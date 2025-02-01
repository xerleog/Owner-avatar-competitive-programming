impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|w| w[0]&1!=w[1]&1)
    }
}
