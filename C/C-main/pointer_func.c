// returning pointer from function
#include <stdio.h>

int* returnPointer(int a[])
{
	a = a + 2;
	return a;
}

void main()
{
	int *p;
	int a[] = {1,2,3,4,5};
	p = returnPointer(a);
	printf("%d", *p);
}

//function pointer
void main(){
	int s = 0;
	
	int (*ptr) (int, int) = &s;
	s = (*ptr)(2,3);
}

int sum (int a, int b);
{
	return a + b;
}

