/*
 * functions.cpp
 *
 *  Created on: Jan 20, 2019
 *      Author: Gabriel
 */

#include "functions.h"
#include <iostream>
#include <iomanip>

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
	cout << "Your final grade is:\t" << fixed << total_grade << endl
		 << "Final letter grade is:\t";

	if (total_grade >= 90)
	{
		cout << "A" << endl;
	}

	else if (total_grade >= 80 and total_grade < 90)
	{
		cout << "B" << endl;
	}

	else if (total_grade >= 70 and total_grade < 80)
	{
		cout << "C" << endl;
	}

	else if (total_grade >= 60 and total_grade < 70)
	{
		cout << "D" << endl;
	}

	else
	{
		cout << "F" << endl;
	}

	cout << endl
		 << endl
		 << endl;
}
