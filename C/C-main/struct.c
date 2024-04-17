#include <stdio.h>

struct student 
{
  int rollno;
  char name[20];
  float marks;
};

//defining quantity of 60 students.
struct student s[60];

int main()
{
  int a;
  struct student s = {1,"bruh", 90.9};

  printf("%d", s.rollno);
 
  //printf("%d", s[0].name);
  //printf("%d", s[0].marks);

}

