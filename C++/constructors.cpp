#include <iostream>

class User 
{
	std::string status = "Gold";

public:
	std::string first_name;
	std::string last_name;
	std::string get_status()
	{
		return status;
	}
	User() //default constructor, has no params.
	{
		std::cout << "constructors \n";
	}
	User(std::string first_name, std::string last_name, std::string status) //additional constructors (has params.)
	{
		this->first_name = first_name;
		this->last_name = last_name;
		this->status = status;
	}
	~User() //destructors.
	{
		std::cout << "Destructor\n";
	}
}

int main()
{
	User user ("bamer", "bfgrswi", "silers");
	std::cout << user.get_status() << std::endl;
}