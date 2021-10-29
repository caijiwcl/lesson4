pub mod add {
    pub fn u32add(numlist:&[u32]) ->Option<u32>{
        let mut result :u32 = 0;
        for element in numlist.iter() {
            let value = match result.checked_add(*element) {
                None => return None,
                Some(r) => r,
            };
            result = value;
        }
        Some(result)
        
    }


}