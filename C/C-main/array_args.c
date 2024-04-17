#include <stdio.h>

void main()
{
	int average;
	int marks[5] = {10, 15, 20, 30, 45};
	
	average = avg (marks, 5);
	printf("avg marks are %d\n", average);
}
int avg (int marks[], int a)
{
	int i, sum = 0, average = 0;
	for(i = 0; i < a; i++)
	{
		sum = sum + marks[i];
	}
	average = sum/a;
	return average;
}