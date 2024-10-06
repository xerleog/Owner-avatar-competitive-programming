impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|a,b| (a&1).cmp(&(b&1)));
        nums
    }
}
