impl Solution {
    pub fn maximum_sum(mut nums: Vec<i32>) -> i32 {
       nums.sort_by(|a,b| b.cmp(&a));
       let mut ans = vec![vec![];82];
        for i in nums.into_iter()
        {
            let mut temp = i as usize;
            let mut sum =0;
            while temp>0
            {
                sum+=(temp%10);
                temp/=10;
            }
            ans[sum].push(i);
        }
        let mut sol = -1;
        for i in 0..82
        {
            if ans[i].len()>=2
            {
                sol =sol.max(ans[i][0]+ans[i][1]);
            }
        }
        sol
    }
}
