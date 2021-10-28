pub mod traffic_light {
    pub trait DuringTime {
         fn  duringtime(&self) -> u32;
    }
    
    pub struct RedLight {
        
    }
    
    impl DuringTime for RedLight {
          fn duringtime(&self) -> u32 {
            60
        }
    }
    
    pub struct GreenLight {
        
    }
    
    impl DuringTime for GreenLight {
         fn duringtime(&self) -> u32 {
            30
        }
    }
    
    pub struct YellowLight {
        
    }
    
    impl DuringTime for YellowLight {
         fn duringtime(&self) -> u32 {
            3
        }
    }

}

