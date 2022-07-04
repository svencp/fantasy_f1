/*
        This is the class for all drivers

        2022.06.27   Sven Ponelat


*/

use crate::library::my_utils::*;
use std::fs::OpenOptions;
use std::io::*;
use std::io::{BufRead};
use std::result::Result;
use termion::{color, style};


#[allow(non_snake_case)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Drivers {
    pub points: i32,
    pub name: String,
    pub team: String,
    pub price: i32,
}

impl Drivers {
    // make an empty Drivers struct
    pub fn new() -> Drivers {
        Drivers {
            name: "".to_string(),
            team: "".to_string(),
            price: 0,
            points: 0,
        }
    }

    // Load all the drivers from text files into map
    #[warn(unused_must_use)]
    pub fn load_driver(d_points_file: &str, d_price_file: &str) -> Result<Vec<Drivers>, String> {
        let mut decoded: Vec<Drivers> = Vec::new();
        let mut last_int: i32 = 0;
        let mut first_float: f32 = 0.0;
        let mut line1: String = "".to_string();
        let mut line2: String = "".to_string();
        let mut last_name: String = "".to_string();
        let mut index: usize = 9999999;

        // Lets open the standings file
        let file_dst = match OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(d_points_file)
        {
            Ok(content) => content,
            Err(_) => {
                return Err("Problem opening driver standings file".to_string());
            }
        };

        let reader1 = BufReader::new(file_dst);
        let mut counter = 1;

        // Main Loop Standings
        // The numbers are parsed staright as integers. They could be negative.
        for line in reader1.lines() {
            if line.is_err() {
                return Err("Something wrong with reader.lines()".to_string());
            }

            let in_string = line.unwrap();

            match counter % 3 {
                1 => {
                    // s_driver = Drivers::new();
                    line1 = in_string.clone();
                }
                2 => {
                    line2 = in_string.clone();
                }
                0 => {
                    let cc = in_string.clone();
                    let chunks: Vec<_> = cc.split_whitespace().collect();
                    for s in chunks {
                        match s.parse::<i32>() {
                            Ok(ii) => last_int = ii,
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                    // Create and assign
                    let mut s_driver = Drivers::new();
                    // let big = make_10x_int(last_float);
                    s_driver.points = last_int;
                    s_driver.name = line1.clone();
                    s_driver.team = line2.clone();
                    decoded.push(s_driver);
                }
                _ => {   //Should never get here, so nothing to do.
                }
            }

            counter += 1;
        }

        // ====================================================== Prices =================================================
        // Now to get the prices inserted
        let file_dpr = match OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(d_price_file)
        {
            Ok(content) => content,
            Err(_) => {
                return Err("Problem opening driver prices file".to_string());
            }
        };

        let reader2 = BufReader::new(file_dpr);
        counter = 1;

        // Main Loop Pricing
        for line in reader2.lines() {
            if line.is_err() {
                return Err("Something wrong with reader.lines()".to_string());
            }

            let in_string = line.unwrap();

            match counter % 3 {
                1 => {
                    // we need to split the line and only get the surname
                    let temp = in_string.clone();
                    let dnames: Vec<_> = temp.split_whitespace().collect();

                    for n in dnames {
                        last_name = n.to_string();
                    }

                    // get the index of driver
                    index = 0;
                    for i in decoded.clone() {
                        if i.name == last_name {
                            break;
                        }
                        index += 1;
                    }
                }
                2 => {
                    // Dont need the team name
                }
                0 => {
                    // Cleanup the string for parsing
                    let cc = in_string.clone();
                    let a1 = cc.replace("$", "");
                    let a2 = a1.replace("m", "");
                    let chunks: Vec<_> = a2.split_whitespace().collect();
                    for s in chunks {
                        match s.parse::<f32>() {
                            Ok(f) => {
                                first_float = f;
                                break;
                            }
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                    // insert the price into decoded
                    let big = make_10x_int(first_float.clone());
                    decoded[index].price = big;
                }
                _ => {   //Should never get here, so nothing to do.
                }
            }

            counter += 1;
        }
        
        decoded.sort_by(|a, b| b.points.cmp(&a.points));
        Ok(decoded)


    } // End of load_driver

    
    
    
} // End of impl



// Function to show driver table
pub fn print_driver_table(table: &Vec<Drivers>){
    const WDRIVER: usize = 12;
    const WPOINTS: usize =  7;
    const WPRICE: usize  =  7;

    let t_driver = "Driver".to_string();
    let t_points = "Points".to_string();
    let t_price = "Price".to_string();

    let tdr = justify(t_driver, WDRIVER, Justify::Left);
    let tpo = justify(t_points, WPOINTS, Justify::Right);
    let tpr = justify(t_price, WPRICE, Justify::Right);
    
    println!();
    println!("{}{} {} {}{}",color::Fg(MY_YELLOW), tdr,tpr,tpo, style::Reset);
    
    for d in table{
        let tdr = justify(d.name.clone(), WDRIVER, Justify::Left);
        let tpr = justify(d.price.to_string(), WPOINTS, Justify::Right);
        let tpo = justify(d.points.to_string(), WPRICE, Justify::Right);
        
        println!("{} {} {}",tdr,tpr,tpo);
    }
}




// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::copy;
    use std::fs::remove_file;

    // #[ignore]
    #[test]
    fn t001_new() {
        let mut dr = Drivers::new();
        dr.name = "Verpy".to_string();

        assert_eq!(dr.name, "Verpy".to_string());
    }

    // #[ignore]
    #[test]
    fn t002_load_1() {
        let res = Drivers::load_driver("./test/points1.txt", "./test/points2.txt");
        assert_eq!(res.is_err(), true);
    }

    // #[ignore]
    #[test]
    fn t003_load_2() {
        let source1 = "./test/store/driver-points.txt";
        let source2 = "./test/store/driver-price.txt";
        let destination1 = "./test/dpo.txt";
        let destination2 = "./test/dpr.txt";
        copy(source1, destination1).expect("Failed to copy");
        copy(source2, destination2).expect("Failed to copy");
        let res = Drivers::load_driver(source1, source2);
        remove_file(destination1).expect("Cleanup test failed");
        remove_file(destination2).expect("Cleanup test failed");

        let r1 = res.clone().unwrap();

        assert_eq!(r1.len(), 20);

        let mut yebo = true;
        for i in res.unwrap() {
            if i.price == 0 {
                yebo = false;
                break;
            }
        }

        assert_eq!(yebo, true);
    }
}
