impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut ans :Vec<i32>= vec![0,0,0];
        moves.chars().for_each(|x| { if x=='L' {ans[0]+=1;} else if x=='R' { ans[1]+=1;} else { ans[2]+=1;}});
        (ans[0]-ans[1]).abs()+ans[2]
    }
}
