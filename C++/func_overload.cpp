#include <iostream>
#include <string>

struct Rectangle
{
	double length;
	double width;
};

double area(double length, double width) //overloaded area function twice.
{
	return length * width;
}

double area(Rectangle rectangle)
{
	return rectangle.length * rectangle.width;
}

double pow(double base, int pow = 2) // default argument.
{
	int total = 1;
	for (int i = 0; i < pow; i++)
	{
		total += base;
	}
	return total;
}

int main()
{
	// Rectangle rectangle;
	// rectangle.length = 10;
	// rectangle.width = 20;

	// std::cout << area(rectangle.length, rectangle.width) << std::endl;
	// std::cout << area(rectangle) << std::endl;

	std::cout << pow(3,3) << std::endl;
	std::cout << pow(3) << std::endl;

	return 0;
}