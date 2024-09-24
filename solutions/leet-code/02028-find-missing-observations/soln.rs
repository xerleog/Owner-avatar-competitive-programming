impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m =rolls.len() as i32;
        let sum :i32= rolls.iter().sum();
        let val = (mean*(m+n))-sum;
        if val<n|| val>6*n { return vec![];}
        let mut ans = vec![val/n;n as usize];
        let mut temp = val%n;
        for i in 0..temp
        {
            ans[i as usize]+=1;
        }
        ans
    }
}
