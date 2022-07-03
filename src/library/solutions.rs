/*
    This is the class for all solutions

    2022.07.02   Sven Ponelat


*/



use crate::library::teams::*;
use crate::library::drivers::*;


#[allow(non_snake_case)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Solutions {
    pub total_points: i32,
    pub total_price: i32,
    pub drivers: Vec<String>,
    pub team: String,
    pub turbo_driver: String,
    pub is_valid: bool,
}



impl Solutions {
    // make an empty Drivers struct
    pub fn new() -> Solutions {
        Solutions {
            total_points: 0,
            total_price: 0,
            is_valid: false,
            drivers: Vec::new(),
            team: "".to_string(),
            turbo_driver: "".to_string(),
        }
    }


    
    
    
    
}// End of impl Solutions




// calcualet the totals and if it is a solution
pub fn calculate_solution(vec: Vec<Drivers>, car: &Teams, budget: i32) -> Solutions {
    let mut ret = Solutions::new();

    for d in vec {
        ret.total_points += d.points;
        ret.total_price += d.price;
        ret.drivers.push(d.name);
    }

    ret.total_points += car.points;
    ret.total_price += car.price;
    ret.team = car.team.clone();

    if ret.total_price <= budget {
        ret.is_valid = true;
    }

    return ret;
} // End of calculate_solution


















