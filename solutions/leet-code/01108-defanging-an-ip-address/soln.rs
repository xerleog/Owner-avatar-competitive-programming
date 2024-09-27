impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut ans = String::new();
        for i in address.split(".")
        {
            ans.push_str(i);
            ans.push_str("[.]");
        }
        ans[..ans.len()-3].to_string()
    }
}
