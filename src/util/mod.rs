use std::{io::{self, Write}, str::FromStr};
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
use either::*;

///Enters terminal alternate screen and takes user input after displaying 'text' and parses the input to the type supplied in 'T'<br>If there is an error during parsing the 'err_handler' value if provided can be used to handle the input.<br>‎‎
///### Parameters:
///```
///text: String
///
///T: Type
///```
///‎<br>‎
///## Examples
///```
///alt_input::<i32>("What is your age", "") // -> i32
///alt_input::<String>("What is your name", "") // -> String
///alt_input::<i32>("What is your grade?", "loop_grades") // -> i32 With custom error handling
///```
pub fn alt_input<T: FromStr>(text: String, err_handler: &str) -> Either<T,  ()> {
    let text_copy: String = text.clone();
    
    let mut user_input: String = String::new();
            
    execute!(io::stdout(), EnterAlternateScreen).expect("Sorry, your computer is not supported.");
    
    execute!(io::stdout(), crossterm::cursor::MoveTo(0,0)).expect("Sorry, your computer is not supported.");
    
    print!("{}: ", text.trim().to_string());
    
    
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut user_input).unwrap();
    
    
    if let Ok(val) = user_input.trim().parse::<T>() {
        execute!(io::stdout(), LeaveAlternateScreen).expect("Sorry, your computer is not supported.");
        return Left(val)
    }
    
    execute!(io::stdout(), LeaveAlternateScreen).expect("Sorry, your computer is not supported.");
    
    match err_handler.to_lowercase().as_str() {
        "loop_grades" => if input_y_or_n("Are you sure you want to stop grading [y/n]?") == true {return Right(())} else {return Left(alt_input::<T>(text_copy, err_handler.to_lowercase().as_str()).unwrap_left())},
        _ => return Left(alt_input::<T>(text_copy, "none").unwrap_left())
    }               

}   


///## Takes user input after displaying 'text' and checks if the input is 'y' or 'n' (Non case-sensitive), if not it restarts the function.
///
///
///### Parameters:
///```
///text: String
///```
///<br/>
///
///## Example:
///```
///input_y_or_n("Do you want to keep playing?") // -> bool
///```
pub fn input_y_or_n(text: &str) -> bool {
    let user_input: String = alt_input::<String>(text.to_string(), "none").unwrap_left().to_lowercase();
    
    if user_input == "y" {
        return true
    }
    else if user_input == "n" {
        return false
    }
    else {
        return input_y_or_n(text)
    }
    
}


///Clears terminal screen (Not OS dependant)<br>‎‎
///## Examples
///```
///clear_console()
///```
pub fn clear_console() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    }
    else {
        std::process::Command::new("clear").status().unwrap();
    }
}
