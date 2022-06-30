/*
        This is the class for all drivers

        2022.06.27   Sven Ponelat


*/

// use std::io::{BufReader,Read, Result};
use std::io::*;
use std::fs::OpenOptions;
// use std::io::Read;
use std::fs::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::Result;



pub const DRIVER_POINTS_FILENAME: &str = "./driver-points.txt";
pub const DRIVER_PRICE_FILENAME: &str = "./driver-price.txt";



#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct Drivers {
    pub name: String,
    pub team: String,
    pub price: f32,
    pub points: f32,
    // pub total: f32
}


impl Drivers {

    // make an empty Drivers struct
    pub fn new() -> Drivers {
        Drivers { 
            name: "".to_string(), 
            team: "".to_string(),  
            price: 0.0,  
            points: 0.0,  
            // total: 0.0,   
        }
    }

    // // calculates total of all points for the driver
    // pub fn total(&mut self) {
    //     let mut t: f32 = 0.0;

    //     for entry in self.points.iter() { 
    //         t = t + entry;
    //     }

    //     self.total = t;
    // }

    // Load all the drivers from text files into map
    #[warn(unused_must_use)]
    pub fn load_driver(dstand_file: &str, dprice_file: &str) -> Result<Vec<Drivers>, String> {
        
        let mut decoded:  Vec<Drivers> = Vec::new();
        // let mut s_driver: Drivers = Drivers::new();
        let mut last_float: f32 = 0.0;
        let mut line1: String = "".to_string();
        let mut line2: String = "".to_string();
        // let mut line3: String = "".to_string();
                
        // Lets open the standings file
        let file_dst = match OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(dstand_file){
                Ok(content) => content,
                Err(_) => { return Err("Problem opening driver standings file".to_string()); }
        };

        let reader = BufReader::new(file_dst);
        let mut counter = 1;

        // Main Loop
        for line in reader.lines() {
            if line.is_err() {
                return Err("Something wrong with reader.lines()".to_string())
            }

            let in_string = line.unwrap();

            match counter % 3 {
                1 =>    {
                            // s_driver = Drivers::new();
                            line1 = in_string.clone();
                        },
                2 =>    {
                            line2 = in_string.clone();
                        },
                0 =>    {
                            let cc = in_string.clone();
                            let chunks: Vec<_> = cc.split_whitespace().collect();
                            for s in chunks {
                                match s.parse::<f32>() {
                                    Ok(f) => { last_float = f  },
                                    Err(e) => { return Err(e.to_string())},
                                }
                            }
                            // Create and assign
                            let mut s_driver = Drivers::new();
                            s_driver.points = last_float;
                            s_driver.name = line1.clone();
                            s_driver.team = line2.clone();
                            decoded.push(s_driver);
                        },
                _ =>    {   //Should never get here, so nothing to do.
                        },

            }

            counter += 1;
        }


        // Now to get the prices inserted









        Ok(decoded)
    }












}







// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
// This was copied with 'use changes' from the Rust manual -> https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html


























// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::copy};
    use std::fs::remove_file;

    
    // #[ignore]
    #[test]
    fn t001_new() {
        let mut dr = Drivers::new();
        dr.name = "Verpy".to_string();

        assert_eq!(dr.name,"Verpy".to_string());
    }


    // #[ignore]
    #[test]
    fn t002_load_1() {

        let res = Drivers::load_driver("./test/points1.txt", "./test/points2.txt");
        assert_eq!(res.is_err(),true);
    }

    
    // #[ignore]
    #[test]
    fn t003_load_2() {

        let source1 = "./test/store/driver-points.txt";
        let source2 = "./test/store/driver-price.txt";
        let destination1 = "./test/dpo.txt";
        let destination2 = "./test/dpr.txt";
        copy(source1,destination1).expect("Failed to copy");
        copy(source2,destination2).expect("Failed to copy");
        let res = Drivers::load_driver(source1, source2);
        remove_file(destination1).expect("Cleanup test failed");
        remove_file(destination2).expect("Cleanup test failed");
        
        assert_eq!(res.unwrap().len(),20);
    }
















}