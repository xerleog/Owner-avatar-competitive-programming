impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let cnt = arr.iter().filter(|&x| *x==0).count();
        if cnt>1 {return true;}
        for i in &arr
        {
            if arr.contains(&(2*i))  && *i!=0
            {   return true;}
        }
        return false;
    }
}
