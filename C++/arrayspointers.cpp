#include <iostream>
#include <limits.h>
// int main(){	

//    int marks[4] = {23, 45, 67, 89}; //arrays. can be done in for, while, dowhile loops.
//    cout << marks[1] << endl;
//    cout << marks[2] << endl;  

//    int * z = marks;
//    cout << "the value of ip is" << *z <<endl; //arrays and pointers
//    return 0; 
// }

// void print_array(int array[], int size){

//    for(int i = 0; i < size; i++)
//    {
//    std::cout << array[i] << "\t";
//    }
//    std::cout << std::endl;

// }

// int main()
// {   
   
//    int guesses[] = {12,34,43,56};
//    int size = sizeof(guesses) / sizeof(guesses[0]);
//    print_array(guesses, 500);

   // int num_elements = 5;

   // std::cout << sizeof(guesses) << std::endl;

   // std::cout << size << std::endl;

   // for(int i = 0; i < num_elements; i++){

   //    std::cout << guesses[i] << "\t";

   // }
   // return 0;
// }

// int main()
// {
//    const int SIZE = 100;
//    int guesses[SIZE];

//    int count = 0;

//    for(int i = 0; i < SIZE; i++)
//    {
//    if(std::cin >> guesses[i])
//    {
//       count++;
//    }
//    else
//    {
//       break;
//    }

//    print_array(guesses, count);
//    std::cin.clear();
//    std::cin.ignore(std::numeric_limits<std::streamsize>::max(),'\n'); //use library limits.h while working with this.

//    }

//    // print_array(guesses, SIZE);

//    return 0;

// }
void print_array(int array[], int count)
{
   for(int i = 0; i < count; i++)
   {
      std::cout << array[i] << "\t";
   }
   std::cout << "\n";
}

void play_game()
{
   int guesses[250];
   int guess_count = 0;

   int random = rand() % 251;
   std::cout << random << std::endl;
   std::cout << "Guess a number: ";
   while(true)
   {
      int guess;
      std::cin >> guess;
      guesses[guess_count++] = guess;
      if(guess == random)
      {
         std::cout << "you win \n";
         break;
      }
      else if (guess < random)
      {
         std::cout << "Too low\n";
      }
      else
      {
         std::cout << "Too high\n";
      }
   } 
   print_array(guesses, guess_count);
}



