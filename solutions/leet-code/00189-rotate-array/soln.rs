impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for i in 0..k
        {
            nums.insert(0,*nums.last().unwrap());
            nums.pop();
        }
    }
}
