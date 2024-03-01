#include <stdio.h>

int main()
{

  int a, b;
  int c = 5;
  int *p;

  a = 4 * (c + 5);

  p = &c;
  b = 4 * (*p + 5);
  printf("a = %d b = %d \n", a, b);
  
}

void ptr() 
{
  int *p1, *p2;
  int i,j;

  p1 = p1 + 1;
  p2 = p1 + j;
  p2++;
  p2 = p2 - (i+j);
  
}
// passing arguments by reference (2 funcs)
int args() 
{

  int a,b;
  a = 5; b = 20;
  
  swap(&a, &b);
  
  printf("\n a = %d, b = %d", a, b);

}

void contd()
{
  int t;

  t = *x;
  *x = *y;
  *y = t;

}

// function to find average

int avr() 
{
  
  int x[100], k, n;

  scanf("%d", &n);

  for (k = 0; k < n; k++)
    scanf("%d", &x[k]);

  printf("\n Average is %f", avg(x,n));

}

float avg (int array[], int size)
{
  int *p, i, sum = 0;

  p = array;

  for(i = 0; i < size; i++)
    sum = sum + *(p+i);

  return ((float)sum / size);
}

#define a
