impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let mut ans = String::new();
        for i in date.split('-')
        {
            ans.push_str((format!("{:b}",i.parse::<i32>().unwrap())).as_str());
            ans.push('-');
        }
        ans.pop();
        ans
    }
}
