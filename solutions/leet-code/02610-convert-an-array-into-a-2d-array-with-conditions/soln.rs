impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans :Vec<Vec<i32>> = vec![];
        for i in nums
        {
            let mut m = true;
            for mut j in &mut ans
            {
                if !j.contains(&i)
                {
                    j.push(i);
                    m=false;
                    break;
                }
            }
            if m
            {   ans.push(vec![i]);}

        }
        ans
    }
}
