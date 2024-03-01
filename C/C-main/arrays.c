// int flag[] = {1,1,1,0};
// char name[] = {'A', 'm', 'g', 'd'};
#include <stdio.h>

int main()
{
	int a[10], i, min;
	printf("Give 10 values\n");
	
	for (i = 0; i < 10; i++)
		scanf("%d", &a[i]);

	min = 9999;
	for (i = 0; i < 10; i++)
	{
		if (a[i] < min)
			min = a[i];
	}
	printf("\n Minimum is \n%d", min);

}

// 2D Arrays
// rows x columns in [] format.

int stud[ 4 ][ 2 ] = {1234, 56, 1212, 33, 1434, 80, 1313, 78};

// 3D Arrays
// Array of arrays of arrays
int arr[3][4][2]={
	{
		{2,4}, {7,8},{3,4},{5,6} 
	},
	{
		{7,6}, {3,4}, {5,3}, {2,3}
	}, 
	{
		{8,9}, {7,2}, {3,4}, {5,1} 
	}
};

