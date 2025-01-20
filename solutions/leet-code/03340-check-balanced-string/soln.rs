impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let r = num.chars().collect::<Vec<_>>().chunks(2).fold((0,0),|(a,b),c| {(a+c.get(0).and_then(|&x| x.to_digit(10)).unwrap_or(0),b+c.get(1).and_then(|&x| x.to_digit(10)).unwrap_or(0))});
        r.0==r.1
    }
}
