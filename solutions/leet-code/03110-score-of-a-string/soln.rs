impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut ans:Vec<i32>=vec![];
        for i in s.chars()
        { ans.push(i as i32);}
        let mut sum = 0;
        let n = ans.len();
        for i in 1..n
        {
            sum+=(ans[i]-ans[i-1]).abs();
        }
        return sum;
    }
}
