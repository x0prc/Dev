#include<stdio.h>
#include<math.h>


// Trapezoidal Rule for Finding Approx Value of Definite Integral
int main()
{
  int n,i;
  double a,b,h, sum = 0, integral;

  printf("\n Enter the number of sub intervals:");
  scanf("%d", &n);

  printf("\n Enter the initial limit:");
  scanf("%If", &a);

  printf("\n Enter the final limit");
  scanf("%If", &b);

  // Integral Computation Begins here
  
  h = fabs(b-a)/n;
  for(i = 1; i < n; i++)
  {
    x = a + i * h;
    sum = sum + f(x);
  }

  integral = (h/2)*(f(a) + f(b) + 2*sum);

  printf("\n The integral is: %If\n", integral);

}
