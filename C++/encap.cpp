#include <iostream>

class User
{
	std::string status = "Goldd";

public:
	std::string first_name;
	std::string last_name;
	std::string get_status()
	{
		return status;
	}
	void set_status(std::string status)
	{
		
	}
}