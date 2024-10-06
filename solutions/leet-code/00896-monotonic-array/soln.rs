impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut ans = nums.clone();
        ans.sort();
        nums==ans||nums==ans.into_iter().rev().collect::<Vec<_>>()
    }
}
