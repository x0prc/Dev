#include <stdio.h>

// int main()
// {

//   int a = 10, b = 5;

//   int *p, *q;

//   p = &a;
//   q = &b;

//   printf("the value of a is %d\n", a);
//   printf("the memory address of a using pointers is %d\n", p);
//   printf("the value of a using pointers is %d\n", *p);

//   printf("the value of b is %d\n", b);
//   printf("the memory address of b using pointers is %d\n", q);
//   printf("the value of b using pointers is %d\n", *q);
// }

// void anotherFunc()
// {
//   // pointer to pointer (double pointer)
//   int f = 15;

//   int *ptr = &f;
//   int **q = &ptr;

//   printf("f = %d %d %d", f, *ptr, **q);
// }

// void arithmeticFunc()
// {
//   // addition
//   int g = 10;

//   int *potr = &g;
//   potr = potr + 2;

//   printf("%d", *potr);

// }
//
// int main()
// {

//   //addition in pointers of array

//   int a [5] = {0, -1, 2, 5, 6};

//   int *p = &a[0];

//   printf("%d\n", *p);
//   p = p + 2;

//   printf("%d\n", *p);

//   return 0;
// }

int main()
{
  char str[] = {"NERD SAY WHAAAAT BRUH MOMENT REPORTED"};
  
  char *ptr = str;
  
  printf("%c", *ptr);

  //operator predence
  printf("%c\n", *(ptr++ +1));
  printf("%c\n", *((ptr-- +5)-1)+3);
  printf("%c\n", *(++ptr + 10)-32);

  //right to left precedence
  printf("%c %c \n", *ptr, *++ptr, *--ptr);
  
  //null pointer
  int *ptr = NULL;

  //dangling pointer [allocated in a free part of memory.]
  free(ptr);

  //wild pointer [shows undefined behavior as nothing is allocated.]
  int *ptr;


}
