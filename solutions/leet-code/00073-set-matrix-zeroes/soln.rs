use std::collections::HashSet;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut col =  HashSet::new(); 
        let n = matrix[0].len();
        for i in 0..matrix.len()
        {
            if matrix[i].contains(&0)
            {
                for j in 0..matrix[i].len()
                {
                    if matrix[i][j]==0
                    {   col.insert(j);}
                }
                matrix[i].clear();
                matrix[i] = vec![0;n];
            }
        }
        for i in &col
        {
            for j in 0..matrix.len()
            {
                matrix[j][*i]=0;
            }
        }
    }
}
