impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        ((start^goal)as i64).count_ones() as i32
    }
}
