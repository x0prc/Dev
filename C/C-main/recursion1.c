#include <stdio.h>

//direct recursion

void display(int n)
{
  if (n < 1){
     return;
  }

  else {
    printf("%d", n);
    
    display(n-1);

    printf("%d", n);
  }

}

int main()
{
  int n = 3;
  display(n);
}

//indirect recursion
void indirect()
{
  printf("%d", fun1(5));
}


int fun1(int n)
{
  if(n <= 1) 
    return 1;
  else
   return n * fun2(n-1);
}


int fun2(int n)
{
  if(n <= 1)
    return 1;
  else
   return n * fun1(n-1);
}

// tail recursion : too much space complexity; use loops instead.
void print(int a)
{
  if (a < 1) 
    return;
  else
   printf("%d", a);
}

void tail()
{
  print(10);
}
