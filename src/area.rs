pub mod area {
    pub trait CalculateArea {
        fn  calculatearea(&self) -> f64;
    }

    pub struct Round {
        pub radius:f64,
    }

    impl CalculateArea for Round {
      fn calculatearea(&self) -> f64 {
        self.radius*self.radius*std::f64::consts::PI
      }
    }

    pub struct Triangle {
        pub bottom:f64,
        pub height:f64,
    }

    impl CalculateArea for Triangle {
      fn calculatearea(&self) -> f64 {
        self.bottom*self.height/2.0
        }
        
    }

    pub struct Square {
        pub length:f64,
    }

    impl CalculateArea for Square {
      fn calculatearea(&self) -> f64 {
        self.length*self.length
        }
    }

    pub fn template_cal_area<T:CalculateArea>(geometric:T) -> f64 {
      geometric.calculatearea()
    }


}


