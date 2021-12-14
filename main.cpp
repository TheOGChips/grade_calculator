#include "Grade.h"
#include <iostream>
#include "functions.h"
#include <list>

using namespace std;

const string SYLLABUS = "syllabus.csv";

int main()
{
    syllabus_t syl;
    read_syllabus(SYLLABUS, syl);
    //print_syllabus(syl);
    
    float total_grade = 0.0;
    unsigned int choice;
    string user_choice;
    char see_grade;
    const int NUM_CATEGORIES = syl.size(),
              DISPLAY_GRADE = NUM_CATEGORIES + 1,
              EXIT = NUM_CATEGORIES + 2;
    
    deque<Grade> grades;
    for (int i = 0; i < NUM_CATEGORIES; i++) {
        string cat = get<0>(syl.front());
        float percentage = get<1>(syl.front()) / 100;
        unsigned int size = get<2>(syl.front());
        string filename = get<3>(syl.front());
        int dropped = get<4>(syl.front());
        
        grades.push_back(Grade(cat, percentage, size, filename, dropped));
        syl.pop();
        add_to_total(total_grade, grades.at(i).get_grade_total());
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
        
        try {
            choice = stoi(user_choice);
        }
        
        catch (invalid_argument exc) {
            choice = 0;
        }
        
        
        if (choice >= 1 and choice <= NUM_CATEGORIES) {
            subtract_from_total(total_grade, grades[choice - 1].get_grade_total());
            try {
                grades[choice - 1].enter_new_score();
            }
            
            catch (out_of_range exc) {
                cerr << endl
                     << endl
                     << "Error: Cannot add anymore grades to this category!" << endl
                     << "       Edit " << SYLLABUS << " if you wish to add more grades" << endl
                     << "       or edit " << grades[choice - 1].get_filename() << " manually" << endl
                     << endl;
            }
            grades[choice - 1].calculate_total();
            add_to_total(total_grade, grades[choice - 1].get_grade_total());
            grades[choice - 1].write_scores_to_file(grades[choice - 1].get_filename());
        }
        
        else if (choice == DISPLAY_GRADE) {
            display_final_grade (total_grade * 100);
        }
        
        else if (choice == EXIT) {
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
    } while (choice != EXIT);
    
    return 0;
}
