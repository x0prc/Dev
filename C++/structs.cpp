#include <iostream>

struct User //like columns of a table.
{
	std::string first_name;
	std::string last_name;
// private:  cannot access from any object.
	std::string get_status(); // can add the same function in a method.
	{
		return status;
	}
};

int main()
{	
	User user;
	user.first_name = "dharampal";
	user.last_name = "chodhri";

	std::cout << "First Name : " << user.first_name << std::endl;

	return 0;

}