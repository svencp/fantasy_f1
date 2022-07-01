/*
A program to do a Linear Program (LP) to find the optimal driver and car combination for a Grand Prix season
up to that point in time.

Because of the inaccuracy of floating point numbers, I have decided to make all numbers to be integers. 
This I feel will make the LP problem more accurite.

2022-06-27    Sven Ponelat
*/


mod library;

use library::drivers::Drivers;
use library::my_utils::*;
use std::process::exit;

use crate::library::teams::Teams;


pub const DRIVER_POINTS_FILENAME: &str = "./driver-points.txt";
pub const DRIVER_PRICE_FILENAME: &str = "./driver-price.txt";
pub const TEAM_POINTS_FILENAME: &str = "./team-points.txt";
pub const TEAM_PRICE_FILENAME: &str = "./team-price.txt";








fn main() {

    // Lets load the files
    let res_driver = Drivers::load_driver(DRIVER_POINTS_FILENAME, DRIVER_PRICE_FILENAME);
    if res_driver.is_err() {
        let message = format!("{}",res_driver.unwrap_err());
        feedback(Feedback::Error, message);
        exit(17);
    }

    let res_team = Teams::load_team(TEAM_POINTS_FILENAME, TEAM_PRICE_FILENAME);
    if res_team.is_err() {
        let message = format!("{}",res_team.unwrap_err());
        feedback(Feedback::Error, message);
        exit(17);
    }

    // The Actual Vectors sorted by points.
    let driver = res_driver.unwrap();
    let team = res_team.unwrap();



    println!("Hello, Svenny!!");
}

































