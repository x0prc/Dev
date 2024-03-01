// #include <stdio.h>

// int main(){

// 	int a,b,c;
	
// 	scanf("%d %d %d", &a, &b, &c); //input variables

// //conditionals

// 	if ((a>b) && (a>c))
// 		printf("Largest answer is \n%d", a);

// 	else 
// 		if (b>c)
// 			printf("Largest is \n%d", b);
// 		else
// 			printf("Largest is \n%d", c);


	
// }

#include <stdio.h>
#define PI 3.1415926

int main() {

	printf("please enter the radius below:\n");
	
	float radius, area;
	float myfunc(float radius); 
	
	scanf ("%f", &radius); //%f --> float
	area = myfunc(radius);
	printf("Area is %f\n", area);
}

float myfunc(float r)
{
	float a;
	a = PI*r*r;
	return(a);
}


/* 
speed --> contents of memory location (used with printf)
&speed --> address of memory location (used with scanf)
*/

// typecasting :

double perimeter;
float pi = 3.14;
int r = 3;

perimeter = 2.0 * (double) pi * (double) r;
printf('perimeter\n');



