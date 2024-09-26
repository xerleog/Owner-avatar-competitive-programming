impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as usize;
        let mut ans :Vec<Vec<i32>> = vec![vec![1];n+1];
        if n>0 { ans[1].push(1);}
        for i in 2..=n
        {
            for j in ans[i-1].clone().windows(2).map(|x| x[0]+x[1])
            {   ans[i].push(j);}
            ans[i].push(1);
        }
        ans[n].clone()
    }
}
