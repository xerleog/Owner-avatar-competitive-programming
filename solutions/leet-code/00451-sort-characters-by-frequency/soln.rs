use std::collections::BTreeMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut ans :BTreeMap<char,i32> = BTreeMap::new();
        for i in s.chars()
        {   *ans.entry(i).or_default()+=1;}
        let mut sol = Vec::from_iter(ans.clone());
        sol.sort_by(|&(_,a),&(_,b)| b.cmp(&a));
        sol.iter().map(|&(x,y)| (0..y).map(|_| x).collect::<String>()).collect::<String>()
    }
}
