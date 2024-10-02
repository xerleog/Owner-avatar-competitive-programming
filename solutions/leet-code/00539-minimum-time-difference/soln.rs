impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut ans = vec![];
        let mut sol = i32::MAX;
        for i in time_points
        {
            let temp = i.split(':').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            ans.push(temp[0]*60+temp[1]);
        }
        ans.sort();
        for j in ans.windows(2)
        {
            sol = sol.min(j[1]-j[0]);
        }
        sol.min(ans[0]-ans[ans.len()-1]+1440)   
    }
}
