use std::{
    fs,
    process,
    str::FromStr,
    io::Write,
};

struct Syllabus {
    categories: Vec<GradeCategory>,
    num_categories: usize,
}

impl<'a> Syllabus {
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
                /*let (name, percent, size, filename, dropped): (String, f32, u8, String, u8) = Self::parse_line(line);
                let category: GradeCategory = GradeCategory::new(name, percent, size, filename, dropped);*/
                let category: GradeCategory = GradeCategory::new(Self::parse_line(line));
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
    
    fn parse_line (line: &str) -> (String, f32, usize, String, u8) {
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

        let size: usize = Self::parse_token::<usize>(tokens.next(), name, "size");
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

struct GradeCategory {
    name: String,         //formerly category
    filename: String,
    percentage: f32,    //formerly percent_grade
    size: usize,           //formerly num_elements
    max_points: u32,
    scores: Vec<f32>,
    total: f32,         //formerly grade_total
    dropped: u8,        //formerly num_dropped_grades
}

impl<'a> GradeCategory {
    fn new ((name, percent, size, filename, dropped): (String, f32, usize, String, u8)) -> GradeCategory {
        let mut category: GradeCategory = GradeCategory {
            name: name,
            filename: filename,
            percentage: percent,
            size: size,
            max_points: (size - dropped as usize) as u32 * 100,
            scores: Vec::new(),
            total: 0.0,         //TODO: Calculate total
            dropped: dropped,
        };
        category.import_scores();
        category.calculate_total();
        return category;
    }

    fn import_scores (&mut self) {  //formerly read_scores_from_file(string)
        match fs::read_to_string(&self.filename) {
            Ok(text) => {
                let mut scores: Vec<f32> = Vec::new();
                for line in text.lines() {
                    scores.push(match line.parse() {
                        Ok(num) => num,
                        Err(msg) => {
                            eprintln!("Error: '{msg}' while reading in scores from {}", self.filename);
                            process::exit(1);
                        }
                    });
                }

                self.scores = scores;

                if self.dropped > 0 {
                    self.sort_scores();
                }

                if self.size != text.lines().count() {
                    self.scores.resize(self.size as usize, -1.0);
                    self.export();
                }
            },
            Err(_) => {
                self.scores = vec![-1.0; self.size as usize];
                self.export();
            }
        }
    }

    fn sort_scores (&mut self) {
        self.scores.sort_by(|a, b| b.partial_cmp(a).unwrap());
    }
    
    fn export (&self) {              //formerly write_scores_to_file(string)
        let scores: Vec<String> = self.scores.iter()
            .map(|score| score.to_string())
            .collect();
        let mut buffer = match fs::File::create(&self.filename) {
            Ok(fp) => fp,
            Err(msg) => {
                eprintln!("Error: '{}' occured while trying to create {}", msg, self.filename);
                process::exit(1);
            }
        };

        for score in scores {
            writeln!(buffer, "{}", score);
        }
    }

    fn calculate_total (&mut self) {
        self.sort_scores();
        self.total = self.scores.iter()
            .map(|&score|
                if score < 0.0 {
                    return 0.0;
                }
                else {
                    return score;
                })
            .reduce(|acc, val| acc + val).unwrap();
        /* NOTE: The only way a None value might happen would be if I removed all the scores from
         *       a file manually, but because of the way I have ensured that the scores files
         *       exist and that sizes are always consistent, this will never happen; unwrapping
         *       is safe here.
         */
        self.total /= self.max_points as f32;
        self.total *= self.percentage;
    }

    fn _enter_new_score () {}

    fn _set_name () {}          //formerly set_category(string)
    fn _set_filename () {}
    fn _set_percentage () {}    //formerly set_percent_grade(float)
    fn _set_size () {}          //formerly set_num_elements(const unsigned int, string)
    fn _line_count () {}
    fn _set_scores () {}
    fn _set_total () {}         //formerly set_grade_total()
    fn _set_dropped () {}       //formerly set_num_dropped_grades(int)

    fn _get_category () {}      //formerly get_category_name() const
    fn _get_filename () {}
    fn _get_percentage () {}    //formerly get_percentage() const
    fn _get_max_points () {}
    fn _get_score () {}
    fn _get_total () {}         //formerly get_grade_total() const
    fn _get_dropped () {}       //formerly get_num_dropped_grades() const
}

fn main() {
    let syllabus: Syllabus = Syllabus::new();
}
