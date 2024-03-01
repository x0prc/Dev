#include <iostream>

void swap(int a, int b)
{
	int temp = a;
	a = b;
	b = a;
	b = temp;

	std::cout << "a: " << a << "\tb:" << b << "\n";

}

int main()
{
	int a = 10;
	int b = 20;

	swap(a,b); //swaps the values to various values stated.

	std::cout << "a: " << a << "\tb:" << b << "\n";
	return 0;

}