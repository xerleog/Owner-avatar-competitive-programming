impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut ans :Vec<Vec<i32>> = vec![];
        let mut sol = vec![];
        let n = score.len();
        let m = score[0].len();
        for i in 0..n
        {
            sol.push(score[i][k as usize]);
        }
        sol.sort();
        sol.reverse();
        for i in sol
        {
            for j in &score
            {
                if j[k as usize]==i
                {   ans.push(j.to_vec());}
            }
        }
        ans

    }
}
