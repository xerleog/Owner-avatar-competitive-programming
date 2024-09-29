impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        for i in queries
        {
            let mut temp = arr[i[1] as usize];
            for j in i[0]..i[1] 
            {   temp^=arr[j as usize];}
            ans.push(temp);
        }
        ans
    }
}
