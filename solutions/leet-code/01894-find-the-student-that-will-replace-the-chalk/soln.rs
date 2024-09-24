impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut i:usize=0;
        let n = chalk.len();
        let mut sum :i64 = chalk.iter().map(|&x| x as i64).sum();
        if n==1 {return 0;}
        let mut val = (k as i64%sum) as i32;
        while val>0 && val>=chalk[(i+n)%n]
        {
            val-=chalk[(i+n)%n];
            i+=1;
        }
        i = (i+n)%n;
        i as i32
    }
}
