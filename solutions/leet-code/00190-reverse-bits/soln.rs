impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let n = format!("{x:032b}");
        let val = n.chars().rev().collect::<String>();
        u32::from_str_radix(val.to_string().as_str(),2).unwrap()
    }
}
