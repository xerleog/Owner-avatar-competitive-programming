impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if m*n != original.len() as i32 { return vec![];}
        original.chunks(n as usize).map(|x| x.to_vec()).collect()
    }
}
