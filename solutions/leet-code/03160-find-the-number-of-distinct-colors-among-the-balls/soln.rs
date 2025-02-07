use std::collections::HashMap;
impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut colors:HashMap<i32,i32> = HashMap::new();
        let mut balls:HashMap<i32,i32> = HashMap::new();
        let mut result = vec![];
        for query in &queries {
            if balls.contains_key(&query[0]) {
                *colors.get_mut(&balls[&query[0]]).unwrap() -= 1;
                if colors[&balls[&query[0]]] == 0 {
                    colors.remove(&balls[&query[0]]);
                }
            }
            *colors.entry(query[1]).or_insert(0) += 1;
            balls.insert(query[0],query[1]);
            result.push(colors.len() as i32);
        }
        result
    }
}
