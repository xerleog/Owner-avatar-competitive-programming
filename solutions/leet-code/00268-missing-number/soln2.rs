impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let total = (n*(n+1))/2;
        let sum = nums.iter().sum::<i32>();
        total-sum
    }
}
