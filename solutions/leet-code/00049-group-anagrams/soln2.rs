use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans:HashMap<String,Vec<String>> = HashMap::new();
        for i in strs
        {
            let mut t :Vec<char> = i.chars().collect();
            t.sort();
            let j:String = t.into_iter().collect();
            ans.entry(j).or_insert_with(Vec::new).push(i);
        }
        ans.into_values().collect()
    }
}
