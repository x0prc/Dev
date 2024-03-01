#include <iostream>
#include <string>
#include "teacher.h"
#include "user.h"



int main()
{
	User user;
	std::cin >> user;
	std::cout << user << std::endl;

	Teacher teacher;
	teacher.output();

}