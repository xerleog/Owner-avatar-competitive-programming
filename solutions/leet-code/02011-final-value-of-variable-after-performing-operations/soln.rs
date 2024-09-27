impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut ans=0;
        for i in operations
        {
            if i=="--X".to_string()|| i =="X--".to_string()
            {   ans-=1;}
            else
            {   ans+=1;}
        }
        ans
    }
}
