#include <string>
#include <iostream>

int main()
{
    int answer = 11;
    std::cout << "Guess the number: ";
    int guess;
    std::cin >> guess;
    
    guess == answer ? std::cout << "Good job\n" : std::cout << "Bad job\n"; //checks whether answer is 10 or not.


}