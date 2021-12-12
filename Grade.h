#ifndef GRADE_H
#define GRADE_H

#include <string>
#include <deque>

using namespace std;

class Grade
{
    private:

        float percent_grade;
        unsigned int num_elements;
        int max_points;
        //float * scores;
        deque<float> scores;
        float grade_total;
        int num_dropped_grades;
        string category;
        string filename;

    public:

        Grade (string, float, unsigned int, string, int);
        ~Grade () {}
        void calculate_total ();
        void bubble_sort ();
        void read_scores_from_file (string);
        void write_scores_to_file (string);
        void enter_new_score ();
        //void dealloc_scores();

        void set_percent_grade (float);
        void set_num_elements (const unsigned int, string);
        unsigned int line_count (string);
        void overwrite_file (const unsigned int, const unsigned int, string);
        void set_max_points ();
        void set_scores (string);
        void set_grade_total ();
        void set_num_dropped_grades (int);
        void set_category (string);
        void set_filename (string);

        float get_percent_grade () const;
        int get_num_elements () const;
        int get_max_points () const;
        float get_score (int) const;
        float get_grade_total () const;
        int get_num_dropped_grades () const;
        string get_category_name () const;
        string get_filename () const;
};

#endif
