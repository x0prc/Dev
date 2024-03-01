#include <iostream>
#include <fstream>
#include <vector>

int main() //writing a file
{
	std::ofstream file ("baller.cpp"); //writing a file into object

	std::vector<std::string> names; 
	names.push_back("#include <iostream>");

	for(std::string name : names)
	{
		file << name << std::endl;
	}


	file.close();
	return 0;
}

int fcc() { //reading a file
	
	std::ifstream file("tacos.txt");
	std::vector<std::string> names;
	std::string input;

	while(file >> input) 
	{
		names.push_back(input);
	}
	for(std::string name : names)
	{
		std::cout << name << std::endl;
	}

	return 0;
}

int cheems() //reading a file.
{
	std::ifstream file ("tacos.txt");

	std::string line;
	getline(file,line);
	std::cout << line << "\n";
	return 0;
}