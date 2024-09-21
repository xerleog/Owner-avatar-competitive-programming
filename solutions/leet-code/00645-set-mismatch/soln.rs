use std::collections::HashMap;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: HashMap<i32,i32> = HashMap::new();
        let mut sol = vec![];
        let n:i32 = nums.len() as i32;
        for i in nums
        {   *ans.entry(i).or_default()+=1;}
        for (val,cnt) in ans.iter()
        {   if *cnt==2 {sol.push(*val);}}
        for j in 1..=n
        {   if ans.get(&j) == None {sol.push(j);}}
        sol
    }
}
