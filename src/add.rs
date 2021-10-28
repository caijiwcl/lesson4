pub mod add {
    pub fn u32add(numlist:&[u32]) ->Option<u32>{
        let mut x : u64 = 0;
        for element in numlist {
            x = x + *element as u64;
        }
        
        if x > u32::MAX.into()
        {
            return None;    
        }
        

        let y:u32 = x as u32;
        Some(y)
        
    }


}