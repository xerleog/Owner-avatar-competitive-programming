impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut seen = std::collections::HashSet::new();
        let mut max_k = -1;
        for num in &nums { seen.insert(*num);}
        for &num in &nums 
        {
            if seen.contains(&-num) 
            { max_k = std::cmp::max(max_k, num.abs());}
        }
        max_k
    }
}
