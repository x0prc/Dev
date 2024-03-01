#include <string>
#include <iostream>
#include <cstdlib>
#include <ctime>

void play_game()
{
	int random = rand() % 251;
	std::cout << random << std::endl;
	std::cout << "guess a number";
	while(true)
	{
		int guess;
		std::cin >> guess;
		if(guess == random)
		{
			std::cout << "You win\n";
			break;
		} else if (guess < random)
		{
			std::cout << "too low\n";
		} else
		{
			std::cout << "too high\n";
		}
	}
}

// int main(){

// 	srand(time(NULL));

// 	int choice;

// 	do
// 	{
// 		std::cout << "0. Quit" << std::endl << "1. Play Game\n";
// 		std::cin >> choice;

// 		switch(choice)
// 		{	
// 		case 0:
// 			std::cout<<"shutup bitchass\n";
// 			break;
// 		case 1:
// 			std::cout<<"yoo lets goo babyyy\n";
// 			break;
// 		}

// 	}
// 	while(choice != 0);

	
// }
