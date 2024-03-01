#include <iostream>
#include <vector>
#include <array>

void test(int data[])
{
	for(int n : data)
	{
		std::cout << n << "\t";
	}
}


int main()
{
	int data[] = {1,2,3,4}; //iterates through every single element in an array.

	for(int n : data)
	{
		std::cout << n << "\t";
	}

	std::cout << "\n";
}