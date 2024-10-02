use std::collections::BTreeSet;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut known =  vec![];
        let mut ans = BTreeSet::new();
        for i in s1.split_whitespace()
        {
            if !ans.contains(&i)&&!known.contains(&i)
            {   ans.insert(i);}
            else if ans.contains(&i)
            {   ans.remove(i);
                known.push(i);
            }
        }
        for j in s2.split_whitespace()
        {
            if ans.contains(&j)
            {   ans.remove(&j);
                known.push(j);}
            else if !known.contains(&j)
            {   ans.insert(&j);}
        }
        ans.iter().map(|x| x.to_string()).collect::<Vec<_>>()
    }
}
