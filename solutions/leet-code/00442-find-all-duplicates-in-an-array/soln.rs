impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        nums.sort();
        for i in 0..nums.len()-1
        {
            if nums[i]==nums[i+1]
            {   ans.push(nums[i]);
            }
        }
        ans
    }
}
