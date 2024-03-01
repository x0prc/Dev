// process by which function calls itself repeatedly.
// for example : factorial, gcd (hcf), fibonacci series
// needs a recursive and stopping condition.
#include <stdio.h>
#include <math.h>

//factorial

long int fact(n)
int n;
{
  if (n == 0)
    return (1);
  else
   return (n * fact(n-1));
}

// fibonacci
long fibonacci (long n)
{
  if (n == 0 || n = 1)
    return n;

  else
   return fibonacci (n - 1) +
      fibonacci (n - 2);
}
