//use crate::traffic_light::*;
use lesson4::traffic_light::traffic_light::*;
use lesson4::add::add::*;
use lesson4::area::area::*;

fn main() {
    let _redlight = RedLight {};
    let _greenlight = GreenLight{};
    let _yellowlight = YellowLight{};


    println!("{}",_redlight.duringtime());


    let number = [40;5];
    let sum = u32add(&number);
    match sum {
        Some(num) => println!("{}",num),
        None => println!("None"),
    }

    let _round = Round{
        radius : 5.0,
    };
    println!("{}",template_cal_area(_round));

}
