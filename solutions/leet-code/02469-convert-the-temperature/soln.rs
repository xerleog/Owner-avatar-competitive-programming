impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let k:f64 = celsius+273.15;
        let f:f64 = celsius*1.80+32.00;
        vec![k,f]
    }
}
