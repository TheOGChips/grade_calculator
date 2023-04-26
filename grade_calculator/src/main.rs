use grade_calculator::Syllabus;
use std::{
    io::{
        stdin,
        stdout,
        Write,
    },
    process::Command,
};
use text_io::read;

fn main() {
    let syllabus: Syllabus = Syllabus::new();
    //TODO: Start working on the menu
    print!("Type something and hit enter: ");
    stdout().flush().unwrap();
    let mut input: String = read!();
    println!("You entered: {}", input);

    //clear_screen();
    print!("Type something else: ");
    input = read!();
    println!("You entered: {}", input);
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
