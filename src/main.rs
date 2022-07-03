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

use crate::library::solutions::Solutions;
use crate::library::teams::*;
// use crate::library::teams::Teams;

pub const DRIVER_POINTS_FILENAME: &str = "./driver-points.txt";
pub const DRIVER_PRICE_FILENAME: &str = "./driver-price.txt";
pub const TEAM_POINTS_FILENAME: &str = "./team-points.txt";
pub const TEAM_PRICE_FILENAME: &str = "./team-price.txt";

fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 3 {
        let message = format!("Not enough arguments, please supply a tenfold budget and turbo price cut-off.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    println!();
    
    
    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& arguments &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&
    // Get budget
    let res_budget = arguments[1].parse::<i32>(); 
    if res_budget.is_err() {
        let message = format!("Budget is not a valid number.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    let budget = res_budget.unwrap();
    println!("The budget is {}",budget);
    
    // Get Turbo price cut-off (tpc)
    let res_tpc = arguments[2].parse::<i32>(); 
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


    // // Try one team first
    // let team1 = &teams[0];

    for t in teams {

        for c in driver_combi.clone() {
            let solution = calculate_solution(c, t.clone(), budget);
            // sol_vec.push(solution);
            if solution.is_valid {
                sol_vec.push(solution.clone());
            }
    
            if solution.total_points > temp_sol.total_points.clone() && solution.is_valid {
                temp_sol = solution.clone();
            }
        }
    }


    // The optimal solutions are now in fronmt of the vector
    sol_vec.sort();
    sol_vec.reverse();


    // // find index
    // let mut index = 0;
    // for f in sol_vec.clone() {
    //     if f.total_points == temp_sol.total_points {
    //         break;
    //     }

    //     index += 1;
    // }


    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&

    // println!("Index of optimal solution is {}",index);
    println!("The higest points were found to be {} with a price of {}",temp_sol.total_points, temp_sol.total_price);
    println!("Hello, Svenny!!");
    show_response(now);



} // End of Main
