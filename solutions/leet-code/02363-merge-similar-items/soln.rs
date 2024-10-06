use std::collections::BTreeMap;
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = BTreeMap::new();
        for i in 0..items1.len()
        {   ans.insert(items1[i][0],items1[i][1]);}
        for i in 0..items2.len()
        {   if let Some(val) = ans.get_mut(&items2[i][0])
            { *val+=items2[i][1];}
            else
            {   ans.insert(items2[i][0],items2[i][1]);}
        }
        ans.into_iter().map(|(a,b)| vec![a,b]).collect::<Vec<_>>()
    }
}
