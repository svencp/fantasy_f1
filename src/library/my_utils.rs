/*  A file to keep functions that I use.

    2022.06.30   Sven Ponelat

*/


use termion::{color, style};



#[allow(dead_code)]
pub enum Feedback{
    Info,
    Warning,
    Error
}



// A function to give command line feedback to situations such as errors or warnings
#[allow(dead_code)]
pub fn feedback(status: Feedback, message: String){
    
    match status {
        Feedback::Info    => { print!("{}{}{}",color::Fg(termion::color::LightYellow),"Info:",style::Reset);}
        Feedback::Warning => { print!("{}{}{}",color::Fg(termion::color::Yellow),"Warning:",style::Reset);}
        Feedback::Error   => { print!("{}{}{}",color::Fg(termion::color::Red),"Error:",style::Reset);}
    }
    print!("  {}\n",message);
}


// Function to convert a float to an intger that is ten times bigger (for accuracy)
pub fn make_10x_int(float: f32) -> i32 {

    let num = float * 10.0;
    return num.round() as i32

}











/*
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
                                    ALL TESTS ARE RUN:  ONE AT A TIME   
                                    
    Running concurrent tests in the same directory with reading and writing has unpredictable results                                    
*/
#[warn(unused_assignments)]
#[cfg(test)]
mod tests {                   //     DONT RUN THE TESTS ABOVE THIS LINE
    use super::*;
    

    // #[ignore]
    #[test]
    fn t001_make_10x_int_1() {
        let f: f32 = 33.4;                                                  // = 33.4000015
        let ans1 = make_10x_int(f);
        
        assert_eq!(ans1,334);
        
        let g: f32 = 1.999982;
        let ans2 = make_10x_int(g);
        
        assert_eq!(ans2,20);
    }


















}