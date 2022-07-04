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
use library::solutions::*;
use std::env;
use std::process::exit;
use std::time::SystemTime;
use termion::{color, style};
use crate::library::solutions::Solutions;
use crate::library::teams::*;



pub const DRIVER_POINTS_FILENAME: &str = "./driver-points.txt";
pub const DRIVER_PRICE_FILENAME: &str = "./driver-price.txt";
pub const TEAM_POINTS_FILENAME: &str = "./team-points.txt";
pub const TEAM_PRICE_FILENAME: &str = "./team-price.txt";

fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 3 {
        let message = format!("Not enough arguments, please supply a tenfold budget first \n
                                    then followed by the turbo price cut-off.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    println!();
    
    
    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& arguments &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&
    // Get budget
    let res_budget = arguments[2].parse::<i32>(); 
    if res_budget.is_err() {
        let message = format!("Budget is not a valid number.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    let budget = res_budget.unwrap();
    println!("The budget is {}",budget);
    
    // Get Turbo price cut-off (tpc)
    let res_tpc = arguments[1].parse::<i32>(); 
    if res_tpc.is_err() {
        let message = format!("Turbo price cut-off is not a valid number.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    let turbo_price_cutoff = res_tpc.unwrap();
    println!("The turbo price cutoff is {}",turbo_price_cutoff);
    println!();
    
    
    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& files &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&
    let res_driver = Drivers::load_driver(DRIVER_POINTS_FILENAME, DRIVER_PRICE_FILENAME);
    if res_driver.is_err() {
        let message = format!("{}", res_driver.unwrap_err());
        feedback(Feedback::Error, message);
        exit(17);
    }

    let res_budget = arguments[1].parse::<i32>();
    if res_budget.is_err() {
        let message = format!(
            "The argument {} could not be parsed into an integer.",
            arguments[1]
        );
        feedback(Feedback::Error, message);
        exit(17);
    }

    let res_team = Teams::load_team(TEAM_POINTS_FILENAME, TEAM_PRICE_FILENAME);
    if res_team.is_err() {
        let message = format!("{}", res_team.unwrap_err());
        feedback(Feedback::Error, message);
        exit(17);
    }

    // The Actual Vectors sorted by points and print the table
    let driver = res_driver.unwrap();
    print_driver_table(&driver);

    // The teams and print
    let teams = res_team.unwrap();
    print_team_table(&teams);

    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& combinatorics &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&

    let mut sol_vec: Vec<Solutions> = Vec::new();
    let mut temp_sol: Solutions = Solutions::new();

    // r = the number of drivers allowed in fantasy
    let r = 5;
    let driver_combi: Vec<_> = Combinations::new(driver.clone(), r).collect();


    for t in teams {

        for c in driver_combi.clone() {
            let sol_td: Vec<Solutions> = calculate_solutions(c, t.clone(), budget, turbo_price_cutoff);

            for solution in sol_td {

                if solution.is_valid {
                    sol_vec.push(solution.clone());
                }
        
                if solution.total_points > temp_sol.total_points.clone() && solution.is_valid {
                    temp_sol = solution.clone();
                }
            }
        }
    }


    // The optimal solutions are now in fronmt of the vector
    sol_vec.sort();
    sol_vec.reverse();


    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& Show Results &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&

    println!("");

    for i in 0..20 {
        let mut color = MY_WHITE;
        if i % 2 == 1 {
            color = MY_WHITER;
        }

        let line = format!("{} {} {} {} {} {}",  sol_vec[i].drivers[0],
                                                        sol_vec[i].drivers[1],
                                                        sol_vec[i].drivers[2],
                                                        sol_vec[i].drivers[3],
                                                        sol_vec[i].drivers[4],
                                                        sol_vec[i].car);
        let just = justify(line, 50, Justify::Left);
        let arr = justify(" --> ".to_string(), 6, Justify::Left);
        let turbo = justify(sol_vec[i].turbo_driver.to_string(), 12, Justify::Right);
        let tpr = justify(sol_vec[i].total_price.to_string(), 7, Justify::Right);
        let tpo = justify(sol_vec[i].total_points.to_string(), 7, Justify::Right);
        println!("{}{}{}{}{}{}{}",color::Fg(color),just,arr,turbo,tpr,tpo,style::Reset);
    }


    println!();
    let f_price: f64 = temp_sol.total_price.to_string().parse::<f64>().unwrap() / 10.0;
    println!("The budget was found to be ${} with the highest points of{}", f_price, temp_sol.total_points);
    show_response(now);


} // End of Main
