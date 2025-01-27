impl Solution {
    pub fn reformat_number(number: String) -> String {
    let digits = number.replace(&[' ','-'],"");
    let mut result = String::new();
    let mut i = 0;
    while i < digits.len() {
        if digits.len() - i > 4 {
            result.push_str(&digits[i..i + 3]);
            result.push('-');
            i += 3;
        } else {
            if digits.len() - i == 4 {
                result.push_str(&digits[i..i + 2]);
                result.push('-');
                result.push_str(&digits[i + 2..]); 
                break;
            }
             else {
                result.push_str(&digits[i..]); 
                break;
            }
        }
    }

    result
}}
