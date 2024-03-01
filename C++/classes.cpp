#include <iostream>

class User
{
	std::string first_name;
	std::string last_name;
	std::string get_status()
	{
		return status;
	}
// private: //no need of private keyword when dealing with classes
	std::string status = "Gold";
}

int add_user_if_not_exists(std::vector<User> &users, User user)
{
	for (int i = 0; i < users.size(); ++i)
	{
		if (users[i].first_name == user.first_name &&
			users[i].last_name == user.last_name)
		{
				return i;
		}
	}
	users.push_back(user);
	return users.size() - 1;
}
int main() ///objects
{
	User user;
	user.first_name = "Gamer";

	std::vector<User> users;
	users.push_back(User());

	std::cout << users[0].first_name << std::endl;
	return 0;

}