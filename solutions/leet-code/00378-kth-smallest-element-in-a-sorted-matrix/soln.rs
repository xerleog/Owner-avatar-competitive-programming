impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = vec![];
        for mut i in matrix
        {   ans.append(&mut i);}
        ans.sort();
        ans[(k-1) as usize]
    }
}
