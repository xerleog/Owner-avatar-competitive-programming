impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut sum = 0;
        for i in s.chars()
        {
            let mut se = (i as i32)-96;
            while se>0 
            {
                sum+=se%10;
                se/=10;
            }
        }
        for _i in 1..k
        {
            let mut temp = 0;
            while sum>0
            {
                temp+=sum%10;
                sum/=10;
            }
            sum=temp;
        }
        sum
    }
}
