use std::{
    fs,
    process,
};

struct Syllabus<'a> {
    categories: Vec<GradeCategory<'a>>,
    num_categories: usize,
}

impl<'a> Syllabus<'a> {
    fn new () -> Self {
        const FILENAME: &str = "syllabus.csv";
        const HEADER_LINE: &str = "category,percent,size,filename,dropped";
        let syllabus: String = match fs::read_to_string(FILENAME) {
            Ok(text) => text,
            Err(msg) => {
                eprintln!("Error: {msg}");
                eprintln!("       Please create a syllabus file -> {}", FILENAME);
                process::exit(1);
            },
        };

        if syllabus.is_empty() {
            eprintln!("Error: {} is empty", FILENAME);
            Self::display_header_format_msg(HEADER_LINE);
            process::exit(1);
        }
        else if syllabus.lines().next().unwrap() != HEADER_LINE {
            eprintln!("Error: {} header line is formatted incorrectly", FILENAME);
            Self::display_header_format_msg(HEADER_LINE);
            process::exit(1);
        }
        //TODO: Add check for properly formatted syllabus entries (probably easier to handle this from struct)
        else {
            let num_categories: usize = syllabus.lines().count() - 1;
            let categories: Vec<GradeCategory> = Vec::with_capacity(num_categories);

            for line in syllabus.lines().skip(1) {
                //TODO: Parse each of the remaining lines and construct new GradeCategory objects
                println!("line: {}", line);
            }

            return Syllabus {
                num_categories: num_categories,
                categories: Vec::new(),
            };
        }
    }

    fn display_header_format_msg (header_line: &str) {
        eprintln!("       Use the following for the header line:");
        eprintln!("           {}\n", header_line);
        eprintln!("       All entries should follow this format.\n");
    }
}

struct GradeCategory<'a> {
    _name: &'a str,         //formerly category
    _filename: &'a str,
    _percentage: f32,    //formerly percent_grade
    _size: u8,           //formerly num_elements
    _max_points: u8,
    _scores: Vec<f32>,
    _total: f32,         //formerly grade_total
    _dropped: u8,        //formerly num_dropped_grades
}

impl<'a> GradeCategory<'a> {
    fn _new () {}
    fn _calculate_total () {}
    fn _bubble_sort () {}
    fn _import () {}            //formerly read_scores_from_file(string)
    fn _export () {}            //formerly write_scores_to_file(string)
    fn _enter_new_score () {}

    fn _set_name () {}          //formerly set_category(string)
    fn _set_filename () {}
    fn _set_percentage () {}    //formerly set_percent_grade(float)
    fn _set_size () {}          //formerly set_num_elements(const unsigned int, string)
    fn _line_count () {}
    fn _overwrite_file () {}
    fn _set_max_points () {}
    fn _set_scores () {}
    fn _set_total () {}         //formerly set_grade_total()
    fn _set_dropped () {}       //formerly set_num_dropped_grades(int)

    fn _get_category () {}      //formerly get_category_name() const
    fn _get_filename () {}
    fn _get_percentage () {}    //formerly get_percentage() const
    fn _get_size () {}          //formerly get_num_elements() const
    fn _get_max_points () {}
    fn _get_score () {}
    fn _get_total () {}         //formerly get_grade_total() const
    fn _get_dropped () {}       //formerly get_num_dropped_grades() const
}

fn main() {
    let syllabus: Syllabus = Syllabus::new();
}
