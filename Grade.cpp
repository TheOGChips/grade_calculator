#include "Grade.h"
#include <fstream>
#include <iostream>
#include <string>

using namespace std;

Grade::Grade (string category, float percent, unsigned int size, string filename, int dropped)
{
    set_percent_grade(percent);
    set_num_elements(size, filename);
    set_num_dropped_grades(dropped);
    set_max_points();
    set_filename(filename);
    set_scores(get_filename());
    set_grade_total();
    set_category(category);
}

void Grade::set_percent_grade (float percent)
{
    percent_grade = percent;
}

void Grade::set_num_elements (const unsigned int SIZE, string filename)
{
    const unsigned int PREV_SIZE = line_count(filename);
    if (PREV_SIZE != SIZE) {
        overwrite_file(PREV_SIZE, SIZE, filename);
    }

    else {
        num_elements = SIZE;
    }
}

unsigned int Grade::line_count (string filename)
{
    unsigned int count = 0;
    ifstream infile;
    infile.open(filename.c_str());

    if (infile.fail()) {    //NOTE: There might be a better place to put this
        return 0;
    }

    while (!infile.eof()) {
        string input;
        getline(infile, input);
        
        if (input != "") {
            count++;
        }
    }

    infile.close();
    return count;
}

void Grade::overwrite_file (const unsigned int PREV_SIZE, const unsigned int SIZE, string filename)
{
    if (PREV_SIZE < SIZE) {     //if there are more scores since before
        num_elements = PREV_SIZE;
        set_scores(filename);
        float temp_scores[SIZE];

        for (int i = 0; i < get_num_elements(); i++) {  //get_num_elements returns the previous size
            temp_scores[i] = scores[i];                 //sync temp_scores with scores (up to scores's size) (use scores directly to transfer over placeholder values)
        }

        num_elements = SIZE;
        for (int i = PREV_SIZE; i < get_num_elements(); i++) {  //get_num_elements returns the new size
            temp_scores[i] = -1;                                //fill in placeholder values for extra indeces
        }

        delete [] scores;                           //reset scores so that it has bigger size
        scores = new float[get_num_elements()];

        for (int i = 0; i < get_num_elements(); i++) {
            scores[i] = temp_scores[i];
        }

        write_scores_to_file(filename);     //overwrite the file
        delete [] scores;
    }

    else {
        num_elements = SIZE;
        set_scores(filename);           //fill in scores up to new lower size
        write_scores_to_file(filename); //overwrite the file
        delete [] scores;               //deallocate scores (will be reallocated later)
    }
}

void Grade::set_max_points ()
{
    max_points = (get_num_elements() - get_num_dropped_grades()) * 100;
}

void Grade::set_scores (string filename)
{
    scores = new float[get_num_elements()];
    read_scores_from_file (filename);
}

void Grade::set_grade_total ()
{
    grade_total = 0.0;
    calculate_total();
}

void Grade::set_num_dropped_grades (int dropped)
{
    num_dropped_grades = dropped;
}

void Grade::set_category (string category)
{
    this->category = category;
}

void Grade::set_filename (string filename)
{
    this->filename = filename;
}

void Grade::calculate_total ()
{
    if (get_num_dropped_grades() > 0) {
        bubble_sort();
    }

    grade_total = 0.0;

    for (int i = 0; i < get_num_elements() - get_num_dropped_grades(); i++) {
        grade_total += get_score(i);
    }

    grade_total = (grade_total / get_max_points()) * get_percent_grade();
}

void Grade::bubble_sort ()
{
    float temp;
    bool no_swaps;

    for (int i = 0; i < get_num_elements() - 1; i++) {
        no_swaps = true;

        for (int j = 0; j < get_num_elements() - 1; j++) {
            if (scores[j] < scores[j + 1]) {
                temp = scores[j];
                scores[j] = scores[j + 1];
                scores[j + 1] = temp;
                no_swaps = false;
            }
        }

        if (no_swaps) {
            return;
        }
    }
}

float Grade::get_percent_grade () const
{
    return percent_grade;
}

int Grade::get_num_elements () const
{
    return num_elements;
}

int Grade::get_max_points () const
{
    return max_points;
}

float Grade::get_score (int element) const
{
    if (scores[element] == -1) {
        return 0;
    }

    else {
        return scores[element];
    }
}

float Grade::get_grade_total () const
{
    return grade_total;
}

int Grade::get_num_dropped_grades () const
{
    return num_dropped_grades;
}

string Grade::get_category_name () const
{
    return category;
}

string Grade::get_filename () const
{
    return filename;
}

void Grade::read_scores_from_file (string filename)
{
    ifstream infile;
    infile.open(filename.c_str());

    if (infile.fail()) {
        for (int i = 0; i < get_num_elements(); i++) {
            scores[i] = -1;
        }

        write_scores_to_file (filename);
    }

    else {
        for (int i = 0; i < get_num_elements(); i++) {
            infile >> scores[i];
        }

        infile.close();
    }
}

void Grade::write_scores_to_file (string filename)
{
    ofstream outfile;
    outfile.open(filename.c_str());

    for (int i = 0; i < get_num_elements(); i++) {
        outfile << scores[i] << endl;
    }

    outfile.close();
}

void Grade::enter_new_score ()
{
    int i = 0;

    cout << "Enter your newest score:\t";

    while (scores[i] != -1 and i < get_num_elements()) {
        i++;
    }

    cin >> scores[i];
}
