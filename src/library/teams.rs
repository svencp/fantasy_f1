/*
        This is the class for all teams

        2022.06.27   Sven Ponelat


*/


use crate::MAX_NUMBER_OF_RACES;
use crate::library::my_utils::*;
use std::fs::OpenOptions;
use std::io::*;
use std::io::{BufRead};
use std::result::Result;
use termion::{color, style};
use std::cmp;




#[allow(non_snake_case)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TeamStandings {
    pub points: i32,
    pub team: String,
    pub price: i32,
    pub races: Vec<i32>,
}

impl TeamStandings {
    // make an empty Teams struct
    pub fn new() -> TeamStandings {
        TeamStandings {
            team: "".to_string(),
            price: 0,
            points: 0,
            races: Vec::with_capacity(MAX_NUMBER_OF_RACES),
        }
    }

    pub fn significant_races(&mut self, form: i32)   {
        let len = self.races.len() as i32;
        let number_of_pops = len - form;
        self.races.reverse();
        
        for _ in 0..number_of_pops {
            self.races.pop();
        }

        let mut points = 0;
        for p in self.races.clone() {
            points += p;
        }

        self.points = points;
    }






} //end of impl TeamStandings












//&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&& Functions &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&

// Function to show team table
pub fn print_team_table(table: &Vec<TeamStandings>){
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


// Load all the Teams from text files
#[warn(unused_must_use)]
pub fn load_complete_team_table(t_points_file: &str, t_price_file: &str) -> Result<Vec<TeamStandings>, String> {
    let mut decoded: Vec<TeamStandings> = Vec::new();
    let mut ret: Vec<TeamStandings> = Vec::new();
    let mut last_int: i32 = 0;
    let mut first_float: f32 = 0.0;
    let mut line1: String = "".to_string();
    // let mut line2: String = "".to_string();
    // let mut last_name: String = "".to_string();
    let mut index: usize = 9999999;
    let mut min_zeros: i32 = MAX_NUMBER_OF_RACES as i32;

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
                let points: Vec<_> = cc.split_whitespace().collect();

                let mut s_team = TeamStandings::new();
                let mut num_zeros: i32 = 0;

                for s in points {
                    // match s.parse::<i32>() {
                    match s.parse::<f64>() {

                        Ok(ii) => { 
                            let int32 = ii.round() as i32;

                            if int32 == 0 {
                            // if ii == 0.0 {
                                num_zeros += 1;
                            }
                            s_team.races.push(int32);
                            last_int = int32;

                        },
                        Err(e) => return Err(e.to_string()),
                    }
                }
                min_zeros = cmp::min(min_zeros, num_zeros);


                // Create and assign
                s_team.points = last_int;
                s_team.team = line1.clone();
                decoded.push(s_team);
            }
            _ => {   //Should never get here, so nothing to do.
            }
        }

        counter += 1;
    }

    // do some data checking
    let temp_len = decoded[0].clone().races.len() as i32;
    if min_zeros == temp_len {
        return Err("No results have been recorded.".to_string());
    }

    let range_end = temp_len - min_zeros -2;
    let i_end = temp_len -1;
    
    // Shortening the race vector to only have results
    for team in decoded {
        let mut revised = TeamStandings::new();
        let mut index = 0;
        let mut r_vec: Vec<i32> = Vec::new();
        
        for race in team.races {
            if index <= range_end {
                r_vec.push(race);
            }
            
            if index == i_end {
                revised.points = race;
            }
            
            index += 1;
        }
        
        revised.team = team.team;
        revised.races = r_vec;
        
        ret.push(revised);
    }




    // ====================================================== Prices =================================================
    // Now to get the prices inserted
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
                for i in ret.clone() {
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
                ret[index].price = big;
            }
            _ => {   //Should never get here, so nothing to do.
            }
        }

        counter += 1;
    }
    
    ret.sort_by(|a, b| b.points.cmp(&a.points));
    Ok(ret)

}// End of load_teams













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
        let mut te = TeamStandings::new();
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
        let res = load_complete_team_table(source1, source2);
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


    // #[ignore]
    #[test]
    fn t003_sig_1() {
        let form = 3;
        let source1 = "./test/store/team-points.txt";
        let source2 = "./test/store/team-price.txt";
        let destination1 = "./test/tpo.txt";
        let destination2 = "./test/tpr.txt";
        copy(source1, destination1).expect("Failed to copy");
        copy(source2, destination2).expect("Failed to copy");
        let res = load_complete_team_table(source1, source2);
        remove_file(destination1).expect("Cleanup test failed");
        remove_file(destination2).expect("Cleanup test failed");

        let r1 = res.clone().unwrap();
        let len = r1.clone().len();

        assert_eq!(len, 10);

        let team1 = &r1.clone()[0];
        let mut ass = team1.clone();

        ass.significant_races(form);

        assert_eq!(ass.points, 123);
    }







} // End of tests












