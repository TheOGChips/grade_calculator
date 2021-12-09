#ifndef FUNCTIONS_H
#define FUNCTIONS_H

#include <string>
#include <queue>
#include <tuple>

using namespace std;
#define grade_t tuple<int, int, string, int>
#define syllabus_t queue<grade_t>

void add_to_total (float&, float);
void subtract_from_total (float&, float);
void display_final_grade (float);
void read_csv (string, syllabus_t&);
void check_headers (string, string);
void header_error_msg ();
void print_syllabus (syllabus_t);
void read_csv_error ();

#endif
