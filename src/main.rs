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

    2022-07-15      Adding form 

    
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


const VERSION: &str            = env!("CARGO_PKG_VERSION");

pub const DRIVER_POINTS_FILENAME: &str = "./driver-points.txt";
pub const DRIVER_PRICE_FILENAME: &str = "./driver-price.txt";
pub const TEAM_POINTS_FILENAME: &str = "./team-points.txt";
pub const TEAM_PRICE_FILENAME: &str = "./team-price.txt";

const TURBO_DRIVER_CUTOFF: usize = 200;
const MAX_NUMBER_OF_ARGUMENTS: usize = 3;
const MAX_NUMBER_OF_RACES: usize = 30;



fn main() {
    let now = SystemTime::now();
    let budget;
    let mut form: i32= 0;
    
    
    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& arguments &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&
    
    let arguments: Vec<String> = env::args().collect();
    let mut command = String::new();
    let mut sub1 = None;
    
    
    // There are no arguments
    if arguments.len() < 2 {
        let message = format!("Not enough arguments, please supply a tenfold budget. \n \
        (eg.) /home/dave/f1_fantasy/fantasy_f1 990");
        feedback(Feedback::Error, message);
        exit(17);
    }
    
    // There are too many arguments
    if arguments.len() > MAX_NUMBER_OF_ARGUMENTS {
        let message = format!("There are too many arguments, try something like, \n \
        (eg.) /home/dave/f1_fantasy/fantasy_f1 990");
        feedback(Feedback::Error, message);
        exit(17);
    }
    
    
    // It seems I need to do this,otherwise temporary variables get dropped
    match arguments.len() {
        2 => {
            command = arguments[1].to_lowercase().trim().to_owned();
        },
        3 => {
            command = arguments[1].to_lowercase().trim().to_owned();
            sub1 = Some(arguments[2].trim().to_owned());
        },
        
        _ => { () }
    }
    
    // The "_" match goes through both arguments
    match command.as_str() {
        "-version"|"-v"|"v"|"version"   => {  
            let message = format!("My Fantasy F1 version:  {}", VERSION);
            feedback(Feedback::Info, message);
            exit(17);
        } //end of version
        
        
        _ => { 
            let possible_first = command.parse::<i32>();
            if possible_first.is_err() {
                let message = format!("Budget value is not a valid integer");
                feedback(Feedback::Error, message);
                exit(17);
            }
            budget = possible_first.unwrap();
            println!("The budget is {}",budget);
            
            // If there is a second argument
            if sub1.is_some(){
                let possible_second = sub1.unwrap().parse::<i32>();
                
                if possible_second.is_err() {
                    let message = format!("Form value is not a valid integer");
                    feedback(Feedback::Error, message);
                    exit(17);
                }
                form = possible_second.unwrap();
            }
        } // end of _
        
    } // end of match
    
    println!("The turbo price cutoff is {}",TURBO_DRIVER_CUTOFF);
    println!("The form was {}.", form);
    
    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& files &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&
    
    let res_driver = load_complete_driver_table(DRIVER_POINTS_FILENAME, DRIVER_PRICE_FILENAME);
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

    let res_team = load_complete_team_table(TEAM_POINTS_FILENAME, TEAM_PRICE_FILENAME);
    if res_team.is_err() {
        let message = format!("{}", res_team.unwrap_err());
        feedback(Feedback::Error, message);
        exit(17);
    }

    // The Actual Vectors sorted by points and print the table
    let mut driver = res_driver.unwrap();
    
    // The teams and print
    let mut teams = res_team.unwrap();
    
    
    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& form  &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&
    
    let number_of_races = driver.clone()[0].races.len() as i32;
    
    // Change to only count significant races
    if form > 0 && form < number_of_races {
        
        // for drivers
        for drv in &mut driver {
            drv.significant_races(form);
        }
        
        // for teams
        for team in &mut teams {
            team.significant_races(form);
        }
    }
    
    
    // print tables after form
    print_driver_table(&driver);
    print_team_table(&teams);


    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& combinatorics &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&
    
    let mut vec_solutions: Vec<Solutions> = Vec::new();
    let mut temp_sol: Solutions = Solutions::new();
    
    // r = the number of drivers allowed in fantasy
    let r = 5;
    let driver_combinations: Vec<_> = Combinations::new(driver.clone(), r).collect();
    // The arc here might not be necessary, but it does have a speed implication (better)
    let arc_driver_combinations = Arc::new(driver_combinations.clone());
    

    // &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& threading &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&
    let (tx, rx): (Sender<Vec<Solutions>>, Receiver<Vec<Solutions>>) = mpsc::channel();
    let mut children = Vec::new();


    for car in teams.clone() {

        // The sender endpoint can be copied
        let thread_tx = tx.clone();
        let combinations = arc_driver_combinations.clone();


        // Each thread will send its id via the channel
        let child = thread::spawn(move || {

            let mut thread_solutions: Vec<Solutions> = Vec::new();

            for drv in combinations.to_vec() {
                let td_solution: Vec<Solutions> = calculate_solutions(drv, car.clone(), 
                                                budget.clone(), TURBO_DRIVER_CUTOFF as i32);
    
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
    println!("The form was {}.", form);

    show_response(now);


} // End of Main


