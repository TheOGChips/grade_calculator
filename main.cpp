#include "Grade.h"
#include <iostream>
#include "functions.h"

using namespace std;

const string SYLLABUS = "syllabus.csv";

int main()
{
    syllabus_t syl;
    read_csv(SYLLABUS, syl);
    print_syllabus(syl);
    return 0;
    
	float total_grade = 0.0;
	char user_choice,
         see_grade;
	const char * quiz_scores = "quiz_scores.txt",
			   * assignment_scores = "assignment_scores.txt",
			   * homework_scores = "homework_scores.txt";

	Grade quizzes (0.4, 2, quiz_scores, 0);
	Grade assignments (0.5, 5, assignment_scores, 0);
	Grade homework (0.1, 2, homework_scores, 0);

	add_to_total (total_grade, quizzes.get_grade_total());
	add_to_total (total_grade, assignments.get_grade_total());
	add_to_total (total_grade, homework.get_grade_total());

	do
	{
		cout << "Which category do you want to add a grade to?" << endl
			 << endl
			 << "1. Quizzes" << endl
			 << "2. Assignments" << endl
			 << "3. Homework" << endl
			 << "4. See final grade" << endl
			 << "5. End program" << endl;
		cin >> user_choice;

		switch (user_choice)
		{
			case '1': subtract_from_total (total_grade, quizzes.get_grade_total());
					  quizzes.enter_new_score();
					  quizzes.calculate_total();
					  add_to_total (total_grade, quizzes.get_grade_total());
					  quizzes.write_scores_to_file(quiz_scores);
					  break;

			case '2': subtract_from_total (total_grade, assignments.get_grade_total());
					  assignments.enter_new_score();
					  assignments.calculate_total();
					  add_to_total (total_grade, assignments.get_grade_total());
					  assignments.write_scores_to_file(assignment_scores);
					  break;

			case '3': subtract_from_total (total_grade, homework.get_grade_total());
					  homework.enter_new_score();
					  homework.calculate_total();
					  add_to_total (total_grade, homework.get_grade_total());
					  homework.write_scores_to_file(homework_scores);
					  break;

			case '4': display_final_grade (total_grade * 100);
					  break;

			case '5': do
					  {
						  cout << "Would you like to see your final grade? (y/n)\t";
						  cin >> see_grade;

						  if (see_grade == 'y' or see_grade == 'Y')
						  {
							  display_final_grade (total_grade * 100);
						  }

						  cout << endl;
					  } while (see_grade != 'y' and see_grade != 'Y' and see_grade != 'n' and see_grade != 'N');

					  cout << "Exiting program..." << endl;
					  break;

			default: cout << "Invalid option!" << endl
						  << "Choose a valid number option from the main menu." << endl;
		}
	} while (user_choice != '5');

	return 0;
}
