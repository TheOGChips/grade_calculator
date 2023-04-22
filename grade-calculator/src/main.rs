use std::{
    fs,
    process,
    str::FromStr,
};

struct Syllabus<'a> {
    categories: Vec<GradeCategory<'a>>,
    num_categories: usize,
}

impl<'a> Syllabus<'a> {
    const FILENAME: &'a str = "syllabus.csv";

    fn new () -> Self {
        const HEADER_LINE: &str = "category,percent,size,filename,dropped";
        let syllabus: String = match fs::read_to_string(Self::FILENAME) {
            Ok(text) => text,
            Err(msg) => {
                eprintln!("Error: {msg}");
                eprintln!("       Please create a syllabus file -> {}", Self::FILENAME);
                process::exit(1);
            },
        };

        if syllabus.is_empty() {
            eprintln!("Error: {} is empty", Self::FILENAME);
            Self::display_header_format_msg(HEADER_LINE);
        }
        else if syllabus.lines().next().unwrap() != HEADER_LINE {
            eprintln!("Error: {} header line is formatted incorrectly", Self::FILENAME);
            Self::display_header_format_msg(HEADER_LINE);
        }
        else if syllabus.lines().count() == 1 {
            eprintln!("Error: {} has no entries after header line.", Self::FILENAME);
            eprintln!("       At least one entry is required.");
            process::exit(1);
        }
        else {
            let num_categories: usize = syllabus.lines().count() - 1;
            let categories: Vec<GradeCategory> = Vec::with_capacity(num_categories);

            for line in syllabus.lines().skip(1) {
                let (name, percent, size, filename, dropped): (String, f32, u8, String, u8) = Self::parse_line(line);
            }

            //TODO: Construct new GradeCategory objects
            /*      if file(s) don't yet exist
             *          then create and populate them with placeholder -1's
             *      else if sizes and line counts don't match
             *          then if size is smaller
             *                  then read in grades up to size
             *               else read in all grades and append -1's up to size
             *          overwrite grades to the same file
             *      else read grades from files into a vector
             */

            return Syllabus {
                num_categories: num_categories,
                categories: Vec::new(),
            };
        }
    }

    fn display_header_format_msg (header_line: &str) -> ! {
        eprintln!("       Use the following for the header line:");
        eprintln!("           {}\n", header_line);
        eprintln!("       All entries should follow this format.\n");
        process::exit(1);
    }

    fn display_entry_parse_err_msg (name: &str) -> ! {
        eprintln!("       Check that '{}' entry is formatted correctly in {}", name,
                  Self::FILENAME);
        process::exit(1);
    }
    
    fn parse_line (line: &str) -> (String, f32, u8, String, u8) {
        let mut tokens: std::str::Split<&str> = line.split(",");
        /* NOTE: Decided to use static here instead of a mutable borrow because count isn't used
         *       anywhere outside this function
         */
        static mut COUNT: u8 = 1;
        unsafe {
            COUNT += 1;
        }

        /* NOTE: Unwrapping is okay here since I just need to check that the category
            *       name isn't empty, and None will never happen.
            */
        let name: &str = tokens.next().unwrap();
        if name.is_empty() {
            unsafe {
                eprintln!("Error: No category name provided for syllabus entry on line {}", COUNT);
            }
            process::exit(1);
        }
        println!("name: {}", name);

        let percent: f32 = Self::parse_token::<f32>(tokens.next(), name, "percentage") / 100.0;
        println!("percent: {}", percent);

        let size: u8 = Self::parse_token::<u8>(tokens.next(), name, "size");
        println!("size: {}", size);

        let filename: &str = match tokens.next() {
            Some("") => {
                eprintln!("Error: No filename provided for syllabus entry '{}'", name);
                Self::display_entry_parse_err_msg(name);
            },
            Some(token) => token,
            None => {
                eprintln!("Error: No filename found for syllabus entry '{}'", name);
                Self::display_entry_parse_err_msg(name);
            },
        };
        println!("filename: {}", filename);

        let dropped: u8 = Self::parse_token::<u8>(tokens.next(), name, "dropped");
        println!("dropped: {}", dropped);
        
        return (name.to_string(), percent, size, filename.to_string(), dropped);
    }

    fn parse_token<T> (token: Option<&str>, name: &str, column: &str) -> T
    where T: FromStr,
          <T as FromStr>::Err: std::fmt::Display {
        return match token {
            Some(token) => match token.parse::<T>() {
                Ok(token) => token,
                Err(msg) => {
                    eprintln!("Encountered error while reading in {} for syllabus entry '{}': \
                               {}", column, name, msg);
                    Self::display_entry_parse_err_msg(name);
                },
            },
            None => {
                eprintln!("Error: Unable to read category entry '{}'.", name);
                Self::display_entry_parse_err_msg(name);
            },
        };
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