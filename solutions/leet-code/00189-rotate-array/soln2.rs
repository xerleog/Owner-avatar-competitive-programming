impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let m = (k as usize)%nums.len(); 
        nums.rotate_right(m);
        }
}
