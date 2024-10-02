use std::collections::BTreeMap;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut ans :BTreeMap<Vec<i32>,i32> = BTreeMap::new();
        let mut sol = 0;
        let mut sol2 = 0;
        for i in dominoes
        {   *ans.entry(i).or_default()+=1;}
        for (ar,id) in ans.clone()
        {
            sol2+=(ans[&ar]*(ans[&ar]-1))/2;
            if ans.get_key_value(&vec![ar[1],ar[0]])!=None && ar[0]!=ar[1]
            {   sol+=ans[&ar]*ans[&vec![ar[1],ar[0]]];}
        }
        sol/2+sol2
    }
}
