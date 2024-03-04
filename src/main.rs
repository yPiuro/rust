mod util;
use crossterm::{self, style, style::Stylize,terminal::{ EnterAlternateScreen, LeaveAlternateScreen}};
use std::{self, io, isize};

fn main() {
    util::clear_console();
    
    let mut grades: Vec<isize> = Vec::new();

    let mut should_grade: bool = true;

    while should_grade {
        let old_len: usize = grades.len();
        add_grade(&mut grades);
        if grades.len() == old_len {
            should_grade = false
        };
    }
    
    print_grades(grades.clone());

    let average: isize = (| a: Vec<isize> | a.iter().sum::<isize>() / a.len() as isize )(grades);
    
    println!("\n{}:{average}", "Average".with(style::Color::Rgb { r: 255, g: 16, b: 240 }).to_string());
    
}   


fn add_grade(arr:&mut Vec<isize>) {
    
    let msg_input: String = "Input a grade\n".with(style::Color::Rgb {r: 57, g: 255, b: 20}).to_string() +  "Enter anything other than a whole number to stop";
    
    let user_input = util::alt_input::<isize>(msg_input, "loop_grades");
    
    if user_input.is_right() {
        return;
    }

    if user_input.unwrap_left() > 100 ||  user_input.unwrap_left() < 0 {
        crossterm::execute!(io::stdout(), EnterAlternateScreen).expect("Sorry your computer is not supported");
        
        crossterm::execute!(io::stdout(), crossterm::cursor::MoveTo(0,0)).expect("Sorry your computer is not supported");
        
        println!("{}: Please input a grade between 0 - 100.", "ERROR".to_string().with(style::Color::Red));
        
        crossterm::execute!(io::stdout(), crossterm::cursor::Hide).expect("Sorry your computer is not supported");
        
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        crossterm::execute!(io::stdout(), crossterm::cursor::Show).expect("Sorry your computer is not supported");
        
        crossterm::execute!(io::stdout(), LeaveAlternateScreen).expect("Sorry your computer is not supported");
            
        return add_grade(arr)
    }
    
    arr.push(user_input.unwrap_left());
    
}

fn print_grades(grades: Vec<isize>) {
    
    for (mut i, el) in grades.iter().enumerate() {
        i += 1;
        println!("Grade #{i}: {el}");
    }  

}