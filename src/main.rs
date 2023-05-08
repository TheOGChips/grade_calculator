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
    theme::{
        Theme,
        BorderStyle,
        Palette,
    },
};
use cursive_aligned_view::Alignable;
use std::rc::Rc;

fn main () {
    /* NOTE: For some reason, the border prints out in a weird black and white color because
     *       I'm printing out letter grades in different colors. I haven't figured out how to
     *       change that yet, and I don't know if it's worth trying to figure out. If I ever
     *       get curious enough, I'll fix it to be the default,  otherwise I'll consider this
     *       finished.
     */
    let theme = Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::default(),
    };

    let mut tui: CursiveRunnable = cursive::default();
    tui.set_theme(theme);

    let syllabus: Rc<Syllabus> = Rc::new(Syllabus::new());

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

    tui.add_layer(Dialog::around(list.align_top_left())
        .title(format!("{} Grade Calculator", syllabus.name()))
        .button("Quit", |s| s.quit())
        .h_align(HAlign::Left)
    );

    tui.run();
}

fn new_grade_prompt (s: &mut Cursive, name: &str, syl: Rc<Syllabus>) {
    let syl_local: Rc<Syllabus> = Rc::clone(&syl);
    let name_local: String = name.to_string();
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
        else if acc >= 60.0 { format!("{}", "D".green().bold()) }
        else { format!("{}", "F".red().bold()) };
    return format!("{:.2} -> {}", acc, letter_grade);
}
