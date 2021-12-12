#include "functions.h"
#include <iostream>
#include <iomanip>
#include <fstream>
#include <cstring>

using namespace std;

void add_to_total (float & total_grade, float category)
{
    total_grade += category;
}

void subtract_from_total (float & total_grade, float category)
{
    total_grade -= category;
}

void display_final_grade (float total_grade)
{
    cout << endl
         << "Your final grade is:\t" << fixed << total_grade << endl
         << "Final letter grade is:\t";

    if (total_grade >= 90) {
        cout << "A" << endl;
    }

    else if (total_grade >= 80 and total_grade < 90) {
        cout << "B" << endl;
    }

    else if (total_grade >= 70 and total_grade < 80) {
        cout << "C" << endl;
    }

    else if (total_grade >= 60 and total_grade < 70) {
        cout << "D" << endl;
    }

    else {
        cout << "F" << endl;
    }

    cout << endl;
}

void read_syllabus (const string FILENAME, syllabus_t & syl)
{
    ifstream infile;
    infile.open(FILENAME.c_str());
    
    if (infile.fail()) {
        cerr << "Error: Unable to open " << FILENAME << endl
             << "Exiting..." << endl;
        exit(1);
    }
    
    string line,
           delims = ",";
    getline(infile, line);
    check_headers(line, delims);
    
    getline(infile, line);
    while (!infile.eof()) {
        grade_t category;
        
        char * temp = strtok(const_cast<char*>(line.c_str()), delims.c_str());
        get<0>(category) = string (temp);
        
        temp = strtok(nullptr, delims.c_str());
        if (temp == nullptr) {
            read_csv_error();
        }
        get<1>(category) = atoi(temp);
        
        temp = strtok(nullptr, delims.c_str());
        if (temp == nullptr) {
            read_csv_error();
        }
        get<2>(category) = atoi(temp);
        
        temp = strtok(nullptr, delims.c_str());
        if (temp == nullptr) {
            read_csv_error();
        }
        get<3>(category) = temp;
        
        temp = strtok(nullptr, delims.c_str());
        if (temp == nullptr) {
            read_csv_error();
        }
        get<4>(category) = atoi(temp);
        
        syl.push(category);
        getline(infile, line);
    }
    
    infile.close();
}

void check_headers (string headers, string delims)
{
    if (headers != "category,percent,size,filename,dropped") {
        header_error_msg();
    }
}

void header_error_msg ()
{
    cerr << "Error: CSV file format incorrect!" << endl
         << "Use the following format for the header line:" << endl
         << "\tcategory,percent,size,filename,dropped" << endl
         << endl
         << "Exiting..." << endl;
    exit(1);
}

void print_syllabus (syllabus_t syl)
{
    while (!syl.empty()) {
        cout << get<0>(syl.front()) << endl
             << get<1>(syl.front()) << endl
             << get<2>(syl.front()) << endl
             << get<3>(syl.front()) << endl
             << get<4>(syl.front()) << endl
             << endl;
        syl.pop();
    }
}

void read_csv_error()
{
    cerr << "Error: Unable to read value from CSV file" << endl
         << "       Ensure that syllabus.csv is filled out correctly."
         << "Exiting..." << endl;
    exit(1);
}
