/*
        This is the class for all teams

        2022.06.27   Sven Ponelat


*/

use crate::library::my_utils::*;
use std::fs::OpenOptions;
use std::fs::*;
use std::io::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::Result;
use termion::{color, style};


#[allow(non_snake_case)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Teams {
    pub points: i32,
    pub team: String,
    pub price: i32,
}

impl Teams {
    // make an empty Teams struct
    pub fn new() -> Teams {
        Teams {
            team: "".to_string(),
            price: 0,
            points: 0,
        }
    }

    // Load all the Teams from text files
    #[warn(unused_must_use)]
    pub fn load_team(t_points_file: &str, t_price_file: &str) -> Result<Vec<Teams>, String> {
        let mut decoded: Vec<Teams> = Vec::new();
        let mut last_float: f32 = 0.0;
        let mut first_float: f32 = 0.0;
        let mut line1: String = "".to_string();
        let mut index: usize = 9999999;

        // Lets open the standings file
        let file_tst = match OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(t_points_file)
        {
            Ok(content) => content,
            Err(_) => {
                return Err("Problem opening team points file".to_string());
            }
        };

        let reader1 = BufReader::new(file_tst);
        let mut counter = 1;

        // Main Loop Standings
        for line in reader1.lines() {
            if line.is_err() {
                return Err("Something wrong with reader.lines()".to_string());
            }

            let in_string = line.unwrap();

            match counter % 2 {
                1 => {
                    line1 = in_string.clone();
                }
                0 => {
                    let cc = in_string.clone();
                    let chunks: Vec<_> = cc.split_whitespace().collect();
                    for s in chunks {
                        match s.parse::<f32>() {
                            Ok(ff) => { last_float = ff },
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                    // Create and assign
                    let mut s_team = Teams::new();
                    s_team.points = last_float.round() as i32;
                    s_team.team = line1.clone();
                    decoded.push(s_team);
                }
                _ => { //Should never get here, so nothing to do.
                }
            }

            counter += 1;
        }

        // ====================================================== Prices =================================================
        // Now to get the prices inserted
        // Lets open the Prices file file
        let file_tpr = match OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(t_price_file)
        {
            Ok(content) => content,
            Err(_) => {
                return Err("Problem opening team prices file".to_string());
            }
        };

        let reader2 = BufReader::new(file_tpr);
        counter = 1;

        // Main Loop Pricing
        for line in reader2.lines() {
            if line.is_err() {
                return Err("Something wrong with reader.lines()".to_string());
            }

            let in_string = line.unwrap();

            match counter % 2 {
                1 => {
                    // we need to split the line and only get the surname
                    let temp = in_string.clone();

                    // get the index of driver
                    index = 0;
                    for i in decoded.clone() {
                        if i.team == temp {
                            break;
                        }
                        index += 1;
                    }
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
                _ => { //Should never get here, so nothing to do.
                }
            }

            counter += 1;
        }
        
        decoded.sort_by(|a, b| b.points.cmp(&a.points));
        Ok(decoded)

    }// End of load_teams











} // End of impl teams



// Function to show team table
pub fn print_team_table(table: &Vec<Teams>){
    const WTEAM: usize = 12;
    const WPOINTS: usize =  7;
    const WPRICE: usize  =  7;

    let t_team = "Team".to_string();
    let t_points = "Points".to_string();
    let t_price = "Price".to_string();

    let tdr = justify(t_team, WTEAM, Justify::Left);
    let tpo = justify(t_points, WPOINTS, Justify::Right);
    let tpr = justify(t_price, WPRICE, Justify::Right);
    
    println!();
    println!("{}{} {} {}{}",color::Fg(MY_YELLOW), tdr,tpr,tpo, style::Reset);
    
    for d in table{
        let ttr = justify(d.team.clone(), WTEAM, Justify::Left);
        let tpr = justify(d.price.to_string(), WPOINTS, Justify::Right);
        let tpo = justify(d.points.to_string(), WPRICE, Justify::Right);
        
        println!("{} {} {}",ttr,tpr,tpo);
    }
} // End of print_team_table













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
        let mut te = Teams::new();
        te.team = "Verpy".to_string();
        
        assert_eq!(te.team, "Verpy".to_string());
        
    }
    
    
    // #[ignore]
    #[test]
    fn t002_load_1() {
        let source1 = "./test/store/team-points.txt";
        let source2 = "./test/store/team-price.txt";
        let destination1 = "./test/tpo.txt";
        let destination2 = "./test/tpr.txt";
        copy(source1, destination1).expect("Failed to copy");
        copy(source2, destination2).expect("Failed to copy");
        let res = Teams::load_team(source1, source2);
        remove_file(destination1).expect("Cleanup test failed");
        remove_file(destination2).expect("Cleanup test failed");

        let r1 = res.clone().unwrap();

        assert_eq!(r1.len(), 10);

        let mut yebo = true;
        for i in res.unwrap() {
            if i.price == 0 {
                yebo = false;
                break;
            }
        }

        assert_eq!(yebo, true);

    } // End of t002_load_1








} // End of tests












