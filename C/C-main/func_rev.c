#include <stdio.h>

// functions are classified into 4 parts
/* no arguement no return
   no arguement with return
   with arguement no return
   with arguement with return
*/

// no arguement no return
void sum(void);

void main()
{
	sum();
}

void sum()
{
	int a = 5, b = 6, sum = 0;
	sum = a + b;
}

// no arguement with return
int sum(void);
void main()
{
	int s;
	s = sum();
	printf("sum is %d", s);
}

//no arguement with return
int sum()
{
	int a, b, sum = 0;
	printf("Enter a and b");
	scanf("%d %d", a, b);
	sum = a + b;
	return sum;
}

// with arguement no return
void sum(float x, float y)
{
	float s = 0;
	s = x + y;
	printf("sum = %f", s);
}

//with arguement with return
void main()
{
	int x, y, c;
	printf("Enter x and y");
	scanf("%d %d", &x, &y);
	c = sum (x, y);
	
	printf("sum = %d", c);
	
}
int sum (int a, int b)
{
	return a + b;
}


// q1 = ans -> 26
int jumble(int x, int y)
{
	x = 2 * x + y;
	return x;
}
int main()
{
	int x = 2, y = 5;
	y = jumble(y, x);
	x = jumble(y, x);
	
	printf("%d\n", x);
	return 0;
}

// q2 = ans ->  value of j is 10
int incr (int i)
{
	static int count = 0;
	count = count + i;
	return (count);
}

int main()
{
	int i, j;
	for(i = 0; i <= 4; i++)
	{
		j = incr(i);
	}
}