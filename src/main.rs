/*
A program to do a Linear Program (LP) to find the optimal driver and car combination for a Grand Prix season
up to that point in time.

Because of the inaccuracy of floating point numbers, I have decided to make all numbers to be integers.
This I feel will make the LP problem more accurite.

    2022-06-27      Sven Ponelat

    2022-07-04      Added Titles to final table
                    Remember to make final execute do: cargo build --release

    2022-07-06      Making the calculations multithreaded
    
    2022-07-08      Adding version to arguments

    
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
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc};
use crate::library::solutions::Solutions;
use crate::library::teams::*;



pub const DRIVER_POINTS_FILENAME: &str = "./driver-points.txt";
pub const DRIVER_PRICE_FILENAME: &str = "./driver-price.txt";
pub const TEAM_POINTS_FILENAME: &str = "./team-points.txt";
pub const TEAM_PRICE_FILENAME: &str = "./team-price.txt";

const VERSION: &str            = env!("CARGO_PKG_VERSION");
const TURBO_DRIVER_CUTOFF: i32 = 200;



fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    // let mut command = None;

    match arguments.len() {
        2 => {
            let command = Some(arguments[1].to_lowercase().trim().to_owned());
            match command.unwrap().as_str() {
                "v"|"V"|"-v"|"-V"|"version"|"Version"|"VERSION"|"-version"|"-Version"|"-VERSION" => {
                    let message = format!("My Fantasy F1 version:  {}", VERSION);
                    feedback(Feedback::Info, message);
                    exit(17);

                }

                    // Not a valid first argument 
                _   => {
                    let message = format!("Not enough arguments, please supply a tenfold turbo price cut-off first \
                                            and then followed by a tenfold budget. \n \
                                            (eg.) /home/dave/f1_fantasy/fantasy_f1 200 990");
                    feedback(Feedback::Error, message);
                    exit(17);
                } //end of _ 
            }
        },

        _ => { () }
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
    
    // // Get Turbo price cut-off (tpc)
    // let res_tpc = arguments[1].parse::<i32>(); 
    // if res_tpc.is_err() {
    //     let message = format!("Turbo price cut-off is not a valid number.");
    //     feedback(Feedback::Error, message);
    //     exit(17);
    // }
    // let turbo_price_cutoff = res_tpc.unwrap();
    // println!("The turbo price cutoff is {}",turbo_price_cutoff);
    // println!();
    
    
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
    
    let mut vec_solutions: Vec<Solutions> = Vec::new();
    let mut temp_sol: Solutions = Solutions::new();
    
    // r = the number of drivers allowed in fantasy
    let r = 5;
    let driver_combinations: Vec<_> = Combinations::new(driver.clone(), r).collect();
    // The arc here might not be necessary
    let arc_driver_combinations = Arc::new(driver_combinations.clone());
    
    
    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& threading &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&
    let (tx, rx): (Sender<Vec<Solutions>>, Receiver<Vec<Solutions>>) = mpsc::channel();
    let mut children = Vec::new();


    for car in teams.clone() {

        // The sender endpoint can be copied
        let thread_tx = tx.clone();
        // let car_name = car.clone().team;
        let combinations = arc_driver_combinations.clone();


        // Each thread will send its id via the channel
        let child = thread::spawn(move || {

            let mut thread_solutions: Vec<Solutions> = Vec::new();

            for drv in combinations.to_vec() {
                let td_solution: Vec<Solutions> = calculate_solutions(drv, car.clone(), 
                                                // budget.clone(), turbo_price_cutoff.clone());
                                                budget.clone(), TURBO_DRIVER_CUTOFF);
    
                for solution in td_solution {
    
                    if solution.is_valid {
                        thread_solutions.push(solution.clone());
                    }
            
                }
            }
            
            
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(thread_solutions).unwrap();
            
            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            // println!("thread {} finished", car_name);
        });
        
        children.push(child);
        
        
    }
    
    // VERY IMPORTANT:
    // Drop the last sender to stop `rx` waiting for message.
    // The program will not complete if we comment this out.
    // **All** `tx` needs to be dropped for `rx` to have `Err`.
    drop(tx);
    
    
    // Here, all the messages are collected
    // let mut ids = Vec::with_capacity(teams.clone().len() as usize);
    for _ in 0..teams.len()  {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        
        // ids.push(rx.recv());
        // instead of pushing, lets work it here
        let th_sol = rx.recv();
        if th_sol.is_err(){
            let message = format!("A thread solution receiver panicked!");
            feedback(Feedback::Error, message);
            exit(17);
        }
        
        for sol in th_sol.unwrap(){
            
            // To check for the highest points
            if sol.total_points > temp_sol.total_points.clone() && sol.is_valid {
                temp_sol = sol.clone();
            }

            vec_solutions.push(sol);
        }
        
    }

    
    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
    
    
    
    
    
    
    
    // The optimal solutions are now in from of the vector
    vec_solutions.sort();
    vec_solutions.reverse();


    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& Show Results &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&

    println!("");
    println!("{}{}                                                    {}        {}   {}{}",
        color::Fg(MY_YELLOW), "Drivers and car","TD","$","Points", style::Reset);

    for i in 0..20 {
        let mut color = MY_WHITE;
        if i % 2 == 1 {
            color = MY_WHITER;
        }

        let line = format!("{} {} {} {} {} {}",  vec_solutions[i].drivers[0],
                                                        vec_solutions[i].drivers[1],
                                                        vec_solutions[i].drivers[2],
                                                        vec_solutions[i].drivers[3],
                                                        vec_solutions[i].drivers[4],
                                                        vec_solutions[i].car);
        let just = justify(line, 54, Justify::Left);
        let arr = justify(" --> ".to_string(), 6, Justify::Left);
        let turbo = justify(vec_solutions[i].turbo_driver.to_string(), 12, Justify::Right);
        let tpr = justify(vec_solutions[i].total_price.to_string(), 7, Justify::Right);
        let tpo = justify(vec_solutions[i].total_points.to_string(), 7, Justify::Right);
        println!("{}{}{}{}{}{}{}",color::Fg(color),just,arr,turbo,tpr,tpo,style::Reset);
    }


    println!();
    let f_price: f64 = temp_sol.total_price.to_string().parse::<f64>().unwrap() / 10.0;
    println!("The budget was found to be ${} with the highest points of {}", f_price, temp_sol.total_points);
    show_response(now);


} // End of Main
