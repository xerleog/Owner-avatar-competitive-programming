impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut  s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        for i in 0..n/2
        {
            if s[i]<s[n-i-1]
            {   s[n-i-1]=s[i];}
            else
            {   s[i]=s[n-i-1];}
        }
        s.iter().collect::<String>()
    }
}
