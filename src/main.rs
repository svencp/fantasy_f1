/*
A program to do a Linear Program (LP) to find the optimal driver and car combination for a Grand Prix season
up to that point in time.

Because of the inaccuracy of floating point numbers, I have decided to make all numbers to be integers. 
This I feel will make the LP problem more accurite.

    2022-06-27    Sven Ponelat
*/


mod library;



use combinations::Combinations;
use library::drivers::*;
use library::my_utils::*;
use std::process::exit;
use std::time::SystemTime;
use std::env;

use crate::library::teams::Teams;


pub const DRIVER_POINTS_FILENAME: &str = "./driver-points.txt";
pub const DRIVER_PRICE_FILENAME: &str = "./driver-price.txt";
pub const TEAM_POINTS_FILENAME: &str = "./team-points.txt";
pub const TEAM_PRICE_FILENAME: &str = "./team-price.txt";








fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        let message = format!("Not enough arguments, please supply a tenfold budget.");
        feedback(Feedback::Error, message);
        exit(17);
    }

    // println!("{}   {:?}",arguments.len(),arguments);


    // Lets load the files
    let res_driver = Drivers::load_driver(DRIVER_POINTS_FILENAME, DRIVER_PRICE_FILENAME);
    if res_driver.is_err() {
        let message = format!("{}",res_driver.unwrap_err());
        feedback(Feedback::Error, message);
        exit(17);
    }

    let res_budget =  arguments[1].parse::<i32>();
    if res_budget.is_err() {
        let message = format!("The argument {} could not be parsed into an integer.",arguments[1]);
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

    // Lets print out the driver vector
    print_driver_table(&driver);




    let team = res_team.unwrap();


    // Lets look at the combinatorics
    let arr = driver.clone();
    let r = 5;

    // let computed: Vec<_> = Combinations::new(arr, r).collect();

    
    println!("Hello, Svenny!!");
    show_response(now);
}

































