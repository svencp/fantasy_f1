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
    pub car: String,
    pub turbo_driver: String,
    pub is_valid: bool,
}



impl Solutions {
    // make an empty Drivers struct
    pub fn new() -> Solutions {
        Solutions {
            total_points: 0,
            total_price: 0,
            drivers: Vec::with_capacity(5),
            turbo_driver: "".to_string(),
            car: "".to_string(),
            is_valid: false,
        }
    }

    
}// End of impl Solutions




// calcualet the totals and if it is a solution
pub fn calculate_solutions(vec: Vec<Drivers>, car: Teams, budget: i32, tdc: i32) -> Vec<Solutions> {

    let mut ret: Vec<Solutions> = Vec::new();
    let mut total_points = 0;
    let mut total_price = 0;
    let mut driver_vec: Vec<String> = Vec::new();


    // Lets get some totals which would be the same for all non turbo drivers
    for d in vec.clone() {
        total_points+= d.points;
        total_price += d.price;
        
        driver_vec.push(d.name);
    }

    
    total_points += car.points; 
    total_price += car.price; 

    
    // Loop through drivers and fill all fields
    for i in 0..vec.len() {
        let mut any_sol: Solutions = Solutions::new();

        any_sol.turbo_driver = vec[i].name.to_string();
        any_sol.total_points = total_points + vec[i].points;
        any_sol.drivers = driver_vec.clone();
        any_sol.car = car.team.clone();
        any_sol.total_price = total_price;

        // Both budget and turbo price cutoof must comply
        if vec[i].price <= tdc && any_sol.total_price <= budget {
            any_sol.is_valid = true;
        }

        ret.push(any_sol);

    }

    return ret;
} // End of calculate_solution


















