impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let ans = s.split('|').collect::<Vec<_>>();
        let mut i =0;
        let mut sol = 0;
        while i<ans.len()
        {
            sol+=ans[i].chars().filter(|&x| x=='*').count();
            i+=2;
        }
        sol as i32
    }
}
