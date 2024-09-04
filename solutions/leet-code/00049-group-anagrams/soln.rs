use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut sol:Vec<Vec<String>>= vec![];
        let mut ans= HashMap::new();
        let mut id:usize=0;
        for i in strs
        {
            let mut t:Vec<char>= i.chars().collect();
            t.sort();
            let j :String = t.into_iter().collect();
            if !ans.contains_key(j.as_str())
            {   
                ans.insert(j,id);
                sol.insert(id,vec![i.to_string()]);
                id+=1;
            }
            else
            {
                sol[*ans.get(j.as_str()).unwrap()].push(i.to_string());
                
            }
        }
        return sol;
    }
}
