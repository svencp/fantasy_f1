/*
        This is the class for all drivers

        2022.06.27   Sven Ponelat


*/


pub const DRIVER_POINTS_FILENAME: &str = "./driver-points.txt";
pub const DRIVER_PRICE_FILENAME: &str = "./driver-price.txt";



#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct Drivers {
    pub name: String,
    pub team: String,
    pub price: f32,
    pub points: Vec<f32>,
    pub total: f32
}


impl Drivers {

    // make an empty Drivers struct
    pub fn new() -> Drivers {
        Drivers { 
                name: "".to_string(), 
                team: "".to_string(),  
                price: 0.0,  
                points: Vec::with_capacity(30),  
                total: 0.0,   
        }
    }

    // calculates total of all points for the driver
    pub fn total(&mut self) {
        let mut t: f32 = 0.0;

        for entry in self.points.iter() { 
            t = t + entry;
        }


        self.total = t;
    }


}

















// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;
    // use std::{fs::copy};
    // use std::fs::remove_file;

    
    // #[ignore]
    #[test]
    fn t001_new() {
        let mut dr = Drivers::new();
        dr.name = "Verpy".to_string();
        dr.points.push(3.0);
        dr.points.push(2.0);

        assert_eq!(dr.points.len(),2);
        
        dr.total();
        assert_eq!(dr.total,5.0);
    }

    // #[ignore]
    #[test]
    fn t002_total_zero_1() {
        let mut dr = Drivers::new();
        dr.name = "Verpy".to_string();       
        dr.total();

        assert_eq!(dr.total,0.0);
    }
    
    // #[ignore]
    #[test]
    fn t002_total_zero_2() {
        let mut dr = Drivers::new();
        dr.name = "Verpy".to_string();       
        dr.total();
        
        assert_eq!(dr.name,"Verpy".to_string());
        
        dr.points.push(2.0);
        assert_eq!(dr.points[0],2.0);
    }




















}