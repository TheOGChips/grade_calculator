use grade_calculator::{
    Syllabus,
    GradeCategory,
};
use cursive::{
    CursiveRunnable,
    views::{
        Dialog,
        LinearLayout,
        TextView,
        DummyView,
        NamedView,
        SelectView,
        EditView,
    },
    view::{
        Nameable,
        Resizable,
    },
    Cursive,
    backends::crossterm::crossterm::style::Stylize,
    align::HAlign,
};
use cursive_aligned_view::Alignable;
use std::{
    rc::Rc,
};
//use colored::Colorize;

fn main () {
    let mut tui: CursiveRunnable = cursive::default();

    let syllabus: Rc<Syllabus> = Rc::new(Syllabus::new());
    //let num_selections: u8 = syllabus.num_categories() + 2;
    //let mut selection: u8 = 0;

    let syl_clone: Rc<Syllabus> = Rc::clone(&syllabus);
    let mut options: SelectView<String> = SelectView::new().on_submit(
        move |s: &mut Cursive, name: &str| {
            let syl_clone: Rc<Syllabus> = Rc::clone(&syl_clone);
            new_grade_prompt(s, name, syl_clone);
        }
    );
    for category in syllabus.categories() {
        options.add_item_str(format!("{}", category.name()));
    }

    let final_grade: NamedView<TextView> = TextView::new(format!("Current course grade: {}", total_grade(Rc::clone(&syllabus)))).with_name("total grade");

    let mut list: LinearLayout = LinearLayout::vertical();
    list.add_child(options);
    list.add_child(DummyView);
    list.add_child(final_grade);

    //TODO: Change some parts of the theme
    tui.add_layer(Dialog::around(list.align_top_left())
        .title(format!("{} Grade Calculator", syllabus.name()))
        .button("Quit", |s| s.quit())
        .h_align(HAlign::Left)
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
    //clearscreen::clear().unwrap();
}

fn new_grade_prompt (s: &mut Cursive, name: &str, syl: Rc<Syllabus>) {
    //TODO: Update total score from here?
    let syl_local: Rc<Syllabus> = Rc::clone(&syl);
    let name_local: String = name.to_string();
    //let name_local: &str = name;
    /*let category: &'static GradeCategory = match syl_local.find(name) {
        Some(cat) => cat,
        None => panic!("Error: Category not found!")
    };*/
    s.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new(format!("Enter a new grade for {}:", name)))
            .child(EditView::new()
                .on_submit(move |s, grade| {
                    match grade.parse::<f32>() {
                        Ok(grade) => if grade >= 0.0 || grade <= 120.0 {
                            let name_local: &str = &name_local;
                            let category: &GradeCategory = syl_local.find(name_local)
                                .unwrap();
                            category.add_grade(grade);
                            category.export();
                            s.call_on_name("total grade",
                                |view: &mut TextView| view.set_content(
                                    format!("Current course grade: {}",
                                            total_grade(Rc::clone(&syl_local))))
                            );
                            s.pop_layer();
                        },
                        Err(_) => (),
                    }
                })
                .fixed_width(6)
            )
    ).button("Back", |s| { s.pop_layer(); }));
}

fn total_grade (syl: Rc<Syllabus>) -> String {
    let mut acc: f32 = 0.0;
    for category in syl.categories() {
        acc += category.total();
    }
    acc *= 100.0;
    let letter_grade: String =
        if acc >= 90.0 { format!("{}", "A".magenta().bold()) }
        else if acc >= 80.0 { format!("{}", "B".green().bold()) }
        else if acc >= 70.0 { format!("{}", "C".green().bold()) }
        else if acc >= 60.0 { format!("{}", "D".yellow().bold()) }
        else { format!("{}", "F".red().bold()) };
    //clear_screen();
    //println!("\nCurrent course grade: {} -> {}", acc, letter_grade);
    return format!("{} -> {}", acc, letter_grade);  //TODO: Format this to 2 decimal places
}
