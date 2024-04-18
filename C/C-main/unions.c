#include <stdio.h>

//structs have difference memory locations for each datatype.
//
struct demo
{
	int a;
	char b;
	float c;
};

void main()
{
	struct demo s = {1, 'a', 4.0};
	printf("%d %c %f", s.a, s.b, s.c);
}

//unions share the same memory locations for different datatypes.

union abc
{
	int a;
	char b;
	float c;
};

void gamer()
{
	union abc u;
	u.a = 1;
	u.b = 97;
	u.c = 9.2;
	printf("a = %d", u.a);
	printf("b = %c", u.b);
	printf("c = %f", u.c);
}
