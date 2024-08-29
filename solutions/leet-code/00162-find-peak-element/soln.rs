impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len()==1
        {return 0;}
        if nums.len()==2
        {
            if(nums[0]>nums[1])
            { return 0;}
            else
            { return 1;}
        }
        let mut val=0;
        let mut id=0;
        for i in 1..nums.len()-1
        {
            if (nums[i-1]<nums[i]) && (nums[i]>nums[i+1])    
            {
                if val<nums[i]
                {   val = nums[i];
                    id=i;
                }
            }
            
        }

        if (nums[0]>nums[1])&&(nums[0]>val)
        {   id = 0;
        }
        if (nums[nums.len()-1]>nums[nums.len()-2])&&(nums[nums.len()-1]>val)
        { id = nums.len()-1; }
        return id as i32;
    }
}
