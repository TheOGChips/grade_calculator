use grade_calculator::{
    Syllabus,
    GradeCategory,
};
use std::{
    process::Command,
    thread::sleep,
    time::Duration,
};
use text_io::read;
use colored::Colorize;
use cursive::{
    CursiveRunnable,
    views::{
        Dialog,
        LinearLayout,
        Button,
    },
    Cursive,
};

fn main() {
    let mut tui: CursiveRunnable = cursive::default();

    clear_screen();
    let syllabus: Syllabus = Syllabus::new();
    let num_selections: u8 = syllabus.num_categories() + 2;
    let mut selection: u8 = 0;

    let mut options: LinearLayout = LinearLayout::vertical();
    for category in syllabus.categories() {
        options.add_child(Button::new(format!("{}", category.1.name()), Cursive::quit));
    }

    //TODO: Read in course name from syllabus file as well to display in menu
    tui.add_layer(Dialog::around(options)
        .title(format!("{} Grade Calculator", "<COURSE NAME>"))
        .button("Quit", Cursive::quit)
    );

    /*while selection != num_selections {
        println!("\n------ MENU ------");
        //NOTE: Display the menu and prompt for user input
        for category in syllabus.categories() {
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

        /* NOTE:
         * if the user input is invalid
         *      then clear the screen and print an error message
         * else act appropriately
         */
        if selection == 0 || selection > num_selections {
            clear_screen();
            //NOTE: For some reason, this doesn't print out when using eprintln.
            println!("\n{}: Invalid option! Choose a number between 1-{}",
                     "Error".red().bold(), num_selections);
        }
        else {
            /* NOTE:
             * if the user selects one of the grade categories
             *      then prompt the user to enter a new grade for the category
             * else if the user selects to print out the current course grade
             *      then do so
             * else
             *      let the loop (and therefore the program) end
             */
            if selection >= 1 && selection <= syllabus.num_categories() {
                let category: &GradeCategory = syllabus.categories()
                    .get(&usize::from(selection))
                    .unwrap();
                /* NOTE:
                 * if the current category is full (no -1 entries left)
                 *      then print error message saying so and continue with next loop iteration
                 * else
                 *      prompt the user to enter a new grade
                 */
                if !category.scores().borrow().contains(&-1.0) {
                        clear_screen();
                        println!("\n{}: Cannot add anymore grades to this category!",
                                 "Error".red().bold());
                        println!("       Edit {} if you wish to add more grades.",
                                 syllabus.filename().cyan());
                }
                else {
                    print!("\nEnter new grade for {}: ", category.name());

                    /* NOTE:
                     * if the user enters a grade outside the valid range of 0-120 or
                     *    the user enters something otherwise invalid
                     *      then print error message saying so and continue with next loop
                     *           iteration
                     * else
                     *      add the grade to the category and write the grades back out to their
                     *      input file
                     */
                    let grade: String = read!();
                    match grade.parse::<f32>() {
                        //NOTE: it's possible to receive an individual grade higher than 100
                        Ok(grade) => if grade < 0.0 || grade > 120.0 {
                            println!("\n{}: Grade value is outside valid range.",
                                     "Error".red().bold());
                            sleep(Duration::from_secs(2));
                        }
                        else {
                            category.add_grade(grade);
                            category.export();
                        },
                        Err(msg) => {
                            println!("\n{}: {}", "Error".red().bold(), msg);
                            sleep(Duration::from_secs(2));
                        },
                    }
                    clear_screen();
                }
            }
            else if selection == num_selections - 1 {
                let mut acc: f32 = 0.0;
                for (_, category) in syllabus.categories() {
                    acc += category.total();
                }
                acc *= 100.0;
                let letter_grade: String =
                    if acc >= 90.0 { format!("{}", "A".purple().bold()) }
                    else if acc >= 80.0 { format!("{}", "B".green().bold()) }
                    else if acc >= 70.0 { format!("{}", "C".green().bold()) }
                    else if acc >= 60.0 { format!("{}", "D".yellow().bold()) }
                    else { format!("{}", "F".red().bold()) };
                clear_screen();
                println!("\nCurrent course grade: {} -> {}", acc, letter_grade);
            }
        }
    }
    println!();*/

    tui.run();
}

/**
 * Source: https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
 * Clears the screen and scrollback history.
 */
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
