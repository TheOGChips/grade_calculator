/*
 * Programs.h
 *
 *  Created on: Jan 16, 2019
 *      Author: Gabriel
 */

#ifndef GRADE_H_
#define GRADE_H_

#include <string>

using namespace std;

class Grade
{
	private:

		float percent_grade;
		unsigned int num_elements;
		int max_points;
		float * scores;
		float grade_total;
		int num_dropped_grades;

	public:

		Grade (float, unsigned int, string, int);
		~Grade();
		void calculate_total();
		void bubble_sort();
		void read_scores_from_file (string);
		void write_scores_to_file (string);
		void enter_new_score();

		void set_percent_grade (float);
		void set_num_elements (const unsigned int, string);
		unsigned int line_count (string);
		void overwrite_file (const unsigned int, const unsigned int, string);
		void set_max_points();
		void set_scores (string);
		void set_grade_total();
		void set_num_dropped_grades (int);

		float get_percent_grade() const;
		int get_num_elements() const;
		int get_max_points() const;
		float get_score (int) const;
		float get_grade_total() const;
		int get_num_dropped_grades() const;
};

#endif /* GRADE_H_ */
