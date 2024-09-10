#include <iostream>
#include <conio.h>

class test
{

  int code;
  static int count;

};

int test :: count;

int main()
{
  test t1, t2;
  
  t1.setcode() // increments the value of t1.
  t1.showcode() // code number of each object.
  t1.showcount() //number of objects created till that moment.

}
