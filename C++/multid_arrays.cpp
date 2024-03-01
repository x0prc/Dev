#include <iostream>
#include <vector>

int main()
{
	std::vector<std::vector<int>> grades = //multi dimensional arrays.
	{
		{1,2,3},
		{5,6,7},
		{9,8,4}
	};

	for(int r = 0; r < 3; r++) //nested for loops.
	{
		for(int c = 0; c < 3; c++)
		{
			std::cout << grades[r][c] << "\t";
		}
		std::cout << "\n";
	}

	return 0;
}