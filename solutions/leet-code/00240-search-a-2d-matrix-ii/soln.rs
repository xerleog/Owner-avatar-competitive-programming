impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for i in matrix
        {
            if i.contains(&target)
            {   return true;}
        }
        return false;
    }
}
