impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = vec![];
        for i in 0..image.len()
        {   ans.push(image[i].clone().into_iter().rev().collect::<Vec<_>>());}
        for i in 0..ans.len()
        {
            for j in 0..ans[0].len()
            {   ans[i][j]^=1;}
        }
        ans
    }
}
