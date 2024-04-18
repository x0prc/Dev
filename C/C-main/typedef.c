#include <stdio.h>

// can be used to declare our own name for struct rather than mentioning struct everytime.
typedef struct student {
	int rollno;
	char name[];
	float marks;
}student;

void main()
{
	student s = {10, "Jenny", 89};
	printf("%d %s %f", s.rollno, s.name, s.marks);
}

