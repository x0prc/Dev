#include <iostream>
#include <string>

// template <typename T> // no need to write the function name again and again.

// void swap(T &a, T &b)
// {
// 	T temp = a;
// 	a = b;
// 	b = a;
// 	b = temp;

// 	std::cout << "a: " << a << "\tb:" << b << "\n";

// }

template <typename T> //overloading templated functions of swap.
void swap (T a[], T b[], int size)
{
	for (int i = 0; i < size; i++)
	{
		T temp = a[i];
		a[i] = b[i];
		b[i] = temp;
	}
}

int main()
{
	int a = 10;
	int b = 20;

	int SIZE = 6;

	swap(a,b); //swaps the values to various values stated.

	std::string first_name = "Gamer";
	std::string last_name = "Baller";

	swap(first_name, last_name);
	std::cout << first_name << " " << last_name << std::endl;

	int nines[] = {9,9,9,9,9,9,9,9};
	int ones[] = {1,1,1,1,1,1,1,1,1};

	for (int i = 0; i < 6; ++i)
	{
		std::cout << nines[i] << "" << ones[i] << "\t";
	}
	
	std::cout << "\n\n";

	swap(nines, ones, SIZE);

	for (int i = 0; i < SIZE; ++i)
	{
		std::cout << nines[i] << " " << ones[i] << "\t";
	}
	std::cout << "\n\n";

	return 0;

}

