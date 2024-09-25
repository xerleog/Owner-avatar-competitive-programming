impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
       let mut sol = vec![]; 
       let mut ans=0;
       for i in bank
       {
            let temp = i.chars().filter(|&x| x=='1').count();
            if temp>0
            { sol.push(temp as i32);}
       }
       if sol.len()<=1 {return 0;}
       for i in 0..sol.len()-1 
        {
            ans+=sol[i]*sol[i+1];
        }
        ans
    }
}
