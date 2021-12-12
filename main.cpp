#include "Grade.h"
#include <iostream>
#include "functions.h"

using namespace std;

const string SYLLABUS = "syllabus.csv";

int main()
{
    syllabus_t syl;
    read_syllabus(SYLLABUS, syl);
    //print_syllabus(syl);
    
    float total_grade = 0.0;
    unsigned int user_choice;
    char see_grade;
    const int NUM_CATEGORIES = syl.size(),
              DISPLAY_GRADE = NUM_CATEGORIES + 1,
              EXIT = NUM_CATEGORIES + 2;
              
    Grade * grades = new Grade[NUM_CATEGORIES];
    for (int i = 0; i < NUM_CATEGORIES; i++) {
        string cat = get<0>(syl.front());
        double percentage = static_cast<double>(get<1>(syl.front())) / 100;
        unsigned int size = get<2>(syl.front());
        string filename = get<3>(syl.front());
        int dropped = get<4>(syl.front());
        
        grades[i] = Grade (cat, percentage, size, filename, dropped);
        syl.pop();
        add_to_total(total_grade, grades[i].get_grade_total());
    }
    
    do
    {
        cout << endl
             << "------ MENU ------" << endl
             << endl;
        
        for (int menu_item = 0; menu_item < NUM_CATEGORIES; menu_item++) {
            cout << menu_item + 1 << ". " << grades[menu_item].get_category_name() << endl;
        }
        cout << DISPLAY_GRADE << ". Display final grade" << endl
             << EXIT << ". Exit" << endl
             << endl        
             << "Which category do you want to add a grade to?\t";
        cin >> user_choice;
        
        if (user_choice >= 1 and user_choice <= NUM_CATEGORIES) {
            subtract_from_total(total_grade, grades[user_choice - 1].get_grade_total());
            grades[user_choice - 1].enter_new_score();
            grades[user_choice - 1].calculate_total();
            add_to_total(total_grade, grades[user_choice - 1].get_grade_total());
            grades[user_choice - 1].write_scores_to_file(grades[user_choice - 1].get_filename());
        }
        
        else if (user_choice == DISPLAY_GRADE) {
            display_final_grade (total_grade * 100);
        }
        
        else if (user_choice == EXIT) {
            do {
                cout << "Would you like to see your final grade? (y/n)\t";
                cin >> see_grade;

                if (see_grade == 'y' or see_grade == 'Y')
                {
                    display_final_grade (total_grade * 100);
                }
            } while (see_grade != 'y' and see_grade != 'Y' and see_grade != 'n' and see_grade != 'N');

            cout << "Exiting program..." << endl
                 << endl;
        }
        
        else {
            cout << "Invalid option!" << endl
                 << "Choose a valid number option from the main menu." << endl;
        }
    } while (user_choice != EXIT);
    
    delete [] grades;
    return 0;
}
