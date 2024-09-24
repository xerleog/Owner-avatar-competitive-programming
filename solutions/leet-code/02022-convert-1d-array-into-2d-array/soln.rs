impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if m*n != original.len() as i32 { return vec![];}
        let mut ans :Vec<Vec<i32>> = vec![vec![];m as usize];
        let mut k :usize= 0;
        for i in 0..m as usize
        {
            for j in 0..n as usize
            {   ans[i].push(original[k]);
                k+=1;
            }
            if k==original.len()
            {   break ;}

        }
        ans

    }
}
