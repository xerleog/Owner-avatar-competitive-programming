impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
     for i in 0..nums.len()
     {
        if nums[i]==target
        {
            return i as i32;
            break;
        }
        else if nums[i]>target
        { return i as i32;}
     }   
    if target < nums[0]
    {return 0 as i32;}
    else
    {return nums.len() as i32;}

    }
}
