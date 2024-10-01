impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut ans :Vec<Vec<i32>> = vec![];
        for i in lo..=hi
        {
            let mut x = i;
            let mut cnt= 0;
            while x!=1
            {
                if x%2==0 { x/=2;}
                else {  x=3*x+1;}
                cnt+=1;
            }
            ans.push(vec![i,cnt]);
        }
        ans.sort_by(|a,b| a[1].cmp(&b[1]));
        ans[(k-1) as usize][0]
    }
}
