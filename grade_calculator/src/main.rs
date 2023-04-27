use grade_calculator::Syllabus;
use std::{
    io::{
        stdin,
        stdout,
        Write,
    },
    process::Command,
    iter::zip,
    thread::sleep,
    time::Duration,
};
use text_io::read;

fn main() {
    clear_screen();
    let syllabus: Syllabus = Syllabus::new();
    //TODO: Also need to calculate total course grade beforehand, maybe?
    //TODO: Start working on the menu
    let num_selections: u8 = syllabus.num_categories() + 2;
    let mut selection: u8 = 0;
    while selection != num_selections {
        println!("\n------ MENU ------");
        //for (category, cat_no) in zip(syllabus.categories(), 1..=syllabus.num_categories()) {
        for category in syllabus.categories() {
            //println!("{}: {}", cat_no, category.name());
            println!("{}: {}", category.0, category.1.name());
        }
        println!("{}: Display final grade", num_selections - 1);
        println!("{}: Exit", num_selections);
        print!("\nWhich category would you like to add a grade to? ");
        let input: String = read!();
        selection = match input.parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if selection == 0 || selection > num_selections {
            //NOTE: For some reason, this doesn't print out when using eprintln.
            println!("\nError: Invalid option! Choose a number between 1-{}\n", num_selections);
            sleep(Duration::from_secs(3));
        }
        else {
            if selection >= 1 && selection <= syllabus.num_categories() {
                if !syllabus.categories()
                    .get(&usize::from(selection))
                    .unwrap()
                    .scores()
                    .contains(&-1.0) {
                        println!("\nError: Cannot add anymore grades to this category!");
                        println!("       Edit {} if you wish to add more grades.",
                                 syllabus.filename());
                        sleep(Duration::from_secs(5));
                }
                else {
                    print!("\nEnter new grade for {}: ",
                           syllabus.categories().get(&usize::from(selection)).unwrap().name());
                    let grade: String = read!();
                    match grade.parse::<f32>() {
                        Ok(num) => if num < 0.0 || num > 120.0 {
                            println!("\nError: Grade value is outside valid range.");
                            sleep(Duration::from_secs(2));
                        }
                        else {
                            todo!(); //TODO: Add the new grade to the GradeCategory's scores Vec
                        },
                        Err(msg) => {
                            println!("\nError: {}", msg);
                            sleep(Duration::from_secs(2));
                        },
                    }
                }
            }
            else if selection == num_selections - 1 {
                //TODO: Calculate and print out the total score for the course
            }
        }
        clear_screen();
    }
    /*print!("Type something and hit enter: ");
    stdout().flush().unwrap();
    let mut input: String = read!();
    println!("You entered: {}", input);

    //clear_screen();
    print!("Type something else: ");
    input = read!();
    println!("You entered: {}", input);
    let num: String = read!();
    match num.parse::<u8>() {
        Ok(n) => println!("num: {}", n),
        Err(msg) => eprintln!("Error: {}", msg),
    };*/
}

//NOTE: Source: https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
fn clear_screen () {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    }
    else if cfg!(target_os = "macos") {
        /* NOTE: Only because clear doesn't behave the same on macOS as it does on Linux by
         *       default. The scrollback history is preserved on macOS, whereas on Linux it isn't
         *       (at least on all the distros I've used).
         */
        let esc: char = 27 as char;
        print!("{}c{}[3J", esc, esc);
    }
    else {
        Command::new("clear").status().unwrap();
    }
}
