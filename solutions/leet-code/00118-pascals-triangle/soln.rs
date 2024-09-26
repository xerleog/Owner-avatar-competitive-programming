impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        if num_rows == 1 { return vec![vec![1]];}
        if num_rows == 2 { return vec![vec![1],vec![1,1]];}
        let mut ans :Vec<Vec<i32>> = vec![vec![1];n];
        ans[1].push(1);
        for i in 2..n
        {
            for j in ans[i-1].clone().windows(2).map(|x| x[0]+x[1])
            {   ans[i].push(j);}
            ans[i].push(1); 
        }
        ans
    }
}
