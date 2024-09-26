impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ans = vec![];
        let mut n = target[target.len()-1];
        for i in 1..=n
        {
            if target.contains(&i)
            {   ans.push("Push".to_string());}
            else
            {
                ans.push("Push".to_string());
                ans.push("Pop".to_string());
            }
        }
        ans
    }
}
