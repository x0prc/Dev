<#include <iostream>

int main (int argc, char *argv[]) {
  
  return 0;
}


class item{

  int number; //private by default
  float cost;

  public:
    void getdata(int a, float b); //inside the class definition
    void putdata(void) {

      cout << number << "\n";
      cout << cost << "\n";

    }


} x, y, z; // xyz are the OBJECTS of type ITEM.


//Outside the Class Definition (member functions)
void item :: getdata(int a, float b)
{
  number = a;
  cost = b;
}

void item :: putdata(void)
{
  cout << number << "\n";
  cout << cost << "\n";
}
