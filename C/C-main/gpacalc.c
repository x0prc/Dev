#include <stdio.h>
#define nsub 10

int main()
{
	int grade_pt[nsub], cred[nsub], i, gp_sum = 0, cred_sum = 0, gpa;
	
	printf("Input Grade Points and Credits for 10 Subjects\n");
		for (i = 0; i < nsub; i++)
			scanf("%d %d", &grade_pt[i], &cred[i]);

		for (i = 0; i < nsub; i++)
		{
			gp_sum += grade_pt[i] * cred[i];
			cred_sum += cred[i];
		}

		gpa = gp_sum / cred_sum;
		printf("\n Grade Point Average is: %d\n", gpa);
}

