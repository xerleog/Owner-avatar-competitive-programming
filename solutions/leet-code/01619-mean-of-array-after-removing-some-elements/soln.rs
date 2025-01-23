impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort();
        let mut sum=0;
        let n = 5*arr.len()/100;
        for i in n..arr.len()-n
        {
            sum+=arr[i];
        }
        sum as f64/(arr.len()-2*n) as f64
    }
}
