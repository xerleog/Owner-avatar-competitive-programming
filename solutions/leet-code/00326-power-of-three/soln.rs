impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let ans =(n as f64).log(3.0);
        ans-(ans as i32)as f64 == 0.0||ans-(ans as i32)as f64 >=0.999999999999990
    }
}
