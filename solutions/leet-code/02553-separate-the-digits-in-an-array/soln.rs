impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for i in nums
        {
            let mut temp = i;
            let mut te = vec![];
            while temp>0
            {
                te.push(temp%10);
                temp/=10;
            }
            ans.extend(te.iter().rev().collect::<Vec<_>>());
        }
        ans
    }
}
