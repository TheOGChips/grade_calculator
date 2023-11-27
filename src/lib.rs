/**
 * A program for calculating one's overall grade for courses based on the course syllabus.
 */
use std::{
    fs,
    process,
    str::FromStr,
    io::Write,
    cell::RefCell,
};

/**
 * The syllabus for the course, containing the breakdown of course grade categories (the
 * percentage each category is worth) overall. Also keeps track of the number of categories.
 */
pub struct Syllabus {
    name: String,
    categories: Vec<GradeCategory>,
    num_categories: usize,
}

impl<'a> Syllabus {
    /// The filename containing the syllabus information. This will always be "syllabus.csv".
    const FILENAME: &'a str = "syllabus.csv";

    /**
     * Creates a new instance of a Syllabus object. Requires the presence of a file named
     * `syllabus.csv` with the header line `category,percent,size,dropped`.
     *
     * `category` -> `String`
     *
     * `percent` -> `f32`
     *
     * `size` -> `usize`
     *
     * `dropped` -> `u8`
     */
    pub fn new () -> Self {
        const HEADER_LINE: &str = "category,percent,size,dropped";
        /* NOTE:
         * if the syllabus file could be read
         *      then return its contents
         * else
         *      print an error that the file (most likely) doesn't exist
         */
        let syllabus: String = match fs::read_to_string(Self::FILENAME) {
            Ok(text) => text,
            Err(msg) => {
                eprintln!("Error: {msg}");
                eprintln!("       Please create a syllabus file -> {}", Self::FILENAME);
                process::exit(1);
            },
        };

        /* NOTE:
         * if the syllabus file was empty
         *      then print an error message saying so
         *           print what the header line should look like
         * else if the header line is wrong
         *      then print an error message saying so
         *           print what the header line should look like
         * else if the syllabus file only has one line (and no entries)
         *      then print an error message saying so and that at least one entry is required
         * else
         *      parse each entry of the syllabus file to create GradeCategory entries
         */
        if syllabus.is_empty() {
            eprintln!("Error: {} is empty", Self::FILENAME);
            Self::display_header_format_msg(HEADER_LINE);
        }
        else if syllabus.lines().skip(1).next().unwrap() != HEADER_LINE {
            eprintln!("Error: {} header lines are formatted incorrectly", Self::FILENAME);
            Self::display_header_format_msg(HEADER_LINE);
        }
        else if syllabus.lines().count() == 1 {
            eprintln!("Error: {} has no entries after header lines.", Self::FILENAME);
            eprintln!("       At least one entry is required.");
            process::exit(1);
        }
        else {
            let name: String = syllabus.lines().next().unwrap().to_string();
            let num_categories: usize = syllabus.lines().count() - 1;
            let mut categories: Vec<GradeCategory> = Vec::new();

            /* NOTE:
             * Parse each line of the syllabus file and pass the resulting tuple directly to
             * create a new GradeCategory object.
             */
            for line in syllabus.lines().skip(2) {
                let category: GradeCategory = GradeCategory::new(Self::parse_line(line));
                categories.push(category);
            }

            return Syllabus {
                name: name,
                num_categories: num_categories,
                categories: categories,
            };
        }
    }

    // Displays an error message if the header line of `syllabus.csv` is improperly formatted.
    fn display_header_format_msg (header_line: &str) -> ! {
        eprintln!("       Use the following for the header lines:");
        eprintln!("           COURSE NAME");
        eprintln!("           {}\n", header_line);
        eprintln!("       All entries should follow this format.\n");
        process::exit(1);
    }

    // Displays an error message if a particular entry in `syllabus.csv` is improperly formatted.
    fn display_entry_parse_err_msg (name: &str) -> ! {
        eprintln!("       Check that '{}' entry is formatted correctly in {}", name,
                  Self::FILENAME);
        process::exit(1);
    }

    /* Parses a line from `syllabus.csv` to read the category name, percentage, number of items
     * in the category, and the number of dropped items for that category (those that won't count
     * towards the final grade calculation).
     */
    fn parse_line (line: &str) -> (String, f32, usize,  u8) {
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

        /* NOTE:
         * if name is empty
         *      then the entry is improperly formatted
         * else
         *      continue parsing each column of the syllabus file
         */
        if name.is_empty() {
            unsafe {
                eprintln!("Error: No category name provided for syllabus entry on line {}", COUNT);
            }
            process::exit(1);
        }
        let percent: f32 = Self::parse_token::<f32>(tokens.next(), name, "percentage") / 100.0;
        let size: usize = Self::parse_token::<usize>(tokens.next(), name, "size");
        let dropped: u8 = Self::parse_token::<u8>(tokens.next(), name, "dropped");
        return (name.to_string(), percent, size, dropped);
    }

    /* Parses a token from the current line being parsed in `syllabus.csv`.
     */
    fn parse_token<T> (token: Option<&str>, name: &str, column: &str) -> T
    where T: FromStr,
          <T as FromStr>::Err: std::fmt::Display {
        /* NOTE:
         * if token exists
         *      then unwrap from Option, parse it, and return it
         * else if an error occurs while parsing the token
         *      print an error saying so with specific information about the current entry
         * else
         *      print error message that the category entry was unable to be read with specific
         *      information about the current entry
         */
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

    /**
     * Returns the name of the course this `Syllabus` was created for. This will be the first
     * line in `syllabus.csv`.
     */
    pub fn name (&self) -> &str {
        return &self.name;
    }

    /**
     * Returns a binary tree map of the list of categories. Keys are unsigned integers in the
     * range `1..Syllabus::size`. Values are `GradeCategory`s.
     */
    pub fn categories (&self) -> &Vec<GradeCategory> {
        return &self.categories;
    }

    /**
     * Returns the number of categories as imported from `syllabus.csv` as a `u8`. This will be
     * the same as the number of lines after the header line of `syllabus.csv`.
     */
    pub fn num_categories (&self) -> u8 {
        return self.num_categories as u8;
    }

    /**
     * Returns the name of the syllabus file. This should always be `syllabus.csv` and is a
     * constant defined within `Syllabus`.
     */
    pub fn filename (&self) -> &str {
        return Self::FILENAME;
    }

    /**
     * Searches for and returns the `GradeCategory` whose `.name` field matches the `name`
     * parameter. This should never return `None`.
     */
    pub fn find (&self, name: &str) -> Option<&GradeCategory> {
        for category in self.categories() {
            if category.name() == name {
                return Some(&category);
            }
        }
        return None;
    }
}

/**
 * Represents a category as defined in `syllabus.csv`. A `RefCell` is used for
 * the `scores` field as it's the only field that will need to be mutable once
 * an instance is created. In other words, the `Vec` of scores will be the only
 * thing that will need to be modified at runtime.
 */
#[derive(Debug)]
pub struct GradeCategory {
    name: String,
    filename: String,
    percentage: f32,
    size: usize,
    max_points: u32,
    scores: RefCell<Vec<f32>>,
    dropped: u8,
}

impl<'a> GradeCategory {
    /* Returns a new GradeCategory instance using the values in the provided
     * tuple.
     */
    fn new ((name, percent, size, dropped): (String, f32, usize, u8)) -> GradeCategory {
        let mut category: GradeCategory = GradeCategory {
            filename: format!("{}.txt", name),
            name: name,
            percentage: percent,
            size: size,
            max_points: (size - dropped as usize) as u32 * 100,
            scores: RefCell::new(Vec::new()),
            dropped: dropped,
        };
        category.import_scores();
        return category;
    }

    /* Imports grades for the category from `self.filename`. The file will
     * contain one grade/score per line. If `filename` doesn't already exist,
     * it will be created and populated with `-1`s, which represent
     * missing/unrecorded grades/scores.
     */
    fn import_scores (&mut self) {
        /* NOTE:
         * Attempt to read in scores from a file. If the file doesn't exist,
         * create it instead.
         */
        match fs::read_to_string(&self.filename) {
            Ok(text) => {
                /* NOTE:
                 * Read and parse each score one line/score at a time. If there
                 * is an error, print the error message and exit.
                 */
                for line in text.lines() {
                    self.scores.borrow_mut().push(match line.parse() {
                        Ok(num) => num,
                        Err(msg) => {
                            eprintln!("Error: '{msg}' while reading in scores from {}", self.filename);
                            process::exit(1);
                        }
                    });
                }

                // NOTE: If this category has dropped scores, sort the Vec.
                if self.dropped > 0 {
                    self.sort_scores();
                }

                /* NOTE:
                 * If the size in syllabus.csv doesn't match the number of
                 * lines in the scores file, then the syllabus.csv file has
                 * been modified since the last time this program was run.
                 * Update the scores file to match syllabus.csv by resizing the
                 * scrores Vec and immediately exporting it.
                 */
                if self.size != text.lines().count() {
                    self.scores.borrow_mut().resize(self.size as usize, -1.0);
                    self.export();
                }
            },
            Err(_) => {
                self.scores.replace(vec![-1.0; self.size as usize]);
                self.export();
            }
        }
    }

    /* Sorts the grades in the Vec from the scores field.
     */
    fn sort_scores (&self) {
        self.scores.borrow_mut().sort_by(|a, b| b.partial_cmp(a).unwrap());
    }

    /**
     * Exports the current grades Vec to `filename`. This always overwrites the
     * previous contents of `filename`.
     */
    pub fn export (&self) {
        //NOTE: For some reason, this doesn't crash the GUI if I sort the scores here
        if self.dropped > 0 {
            self.sort_scores();
        }

        /* NOTE:
         * Borrow the RefCell and return a Vec with the values converted to
         * Strings for writing to a file later.
         */
        let scores: Vec<String> = self.scores.borrow()
            .iter()
            .map(|score| score.to_string())
            .collect();
            
        /* NOTE:
         * Attempt to create a buffer from filename. Print out an error message
         * and exit if an error occurs.
         */
        let mut buffer = match fs::File::create(&self.filename) {
            Ok(fp) => fp,
            Err(msg) => {
                eprintln!("Error: '{}' occured while trying to create {}", msg, self.filename);
                process::exit(1);
            }
        };

        for score in scores {
            writeln!(buffer, "{}", score).unwrap();
        }
    }

    /**
     * Calculates the total score this category will contribute to the overall
     * course grade using the scores from `filename`. This will return a number
     * between \[0, 1\]. The accumulated scores from all categories for a course
     * will yield 1.0 for a perfect final grade in the course.
     */
    pub fn total (&self) -> f32 {
        /* NOTE:
         * Accumulate a category total by iterating through the scores Vec,
         * mapping -1 -> 0 for adding to the accumulator.
         */
        let not_dropped: usize = self.scores.borrow().len() - self.dropped as usize;
        let mut total: f32 = self.scores.borrow()[..not_dropped]
            .iter()
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

        /* NOTE:
         * Divide the accumulated value by the number of max points possible
         * for the category (number of entries in the file * 100). Then,
         * multiply that value by the percentage the category is worth to get
         * the value this category is currently contributing to the final
         * course grade. Return this final value.
         */
        total /= self.max_points as f32;
        total *= self.percentage;
        return total;
    }

    /**
     * Adds a grade the scores `Vec` . The new grade will replace the next `-1` in the `Vec`. The
     * possibility of no -1s is handled prior to this function being called.
     */
    pub fn add_grade (&self, grade: f32) {
        let scores: &mut Vec<f32> = &mut self.scores().borrow_mut();
        let index: Option<usize> = scores.iter().position(|&x| x == -1.0);
        if let Some(index) = index {
            scores[index] = grade;
        }
        //TODO: Signal to print out error message somehow
    }

    /**
     * Returns the name of the category as recorded in syllabus.csv.
     */
    pub fn name (&self) -> &str {
        return &self.name;
    }

    /**
     * Returns a reference to the `RefCell` containing the `Vec` of scores for
     * this category.
     */
    pub fn scores (&self) -> &RefCell<Vec<f32>> {
        return &self.scores;
    }
}
