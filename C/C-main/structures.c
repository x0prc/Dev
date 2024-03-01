#include <stdio.h>


// complex numbers
int main()
{

  struct complex 
  {
    float real;
    float complex;
  } a,b,c;

  scanf("%f %f", &a.real, &a.complex);
  scanf("%f %f", &b.real, &b.complex);

  c.real = a.real + b.real;
  c.complex = a.complex + b.complex;

  printf("\n %f + %f j", c.real, c.complex);
}

// typedef struct -- define a structure datatype with a single name.
typedef struct {
  
  float real;
  float imag;

} _Complex;

void swap(_Complex a, _Complex b)
{
  _Complex tmp;

  tmp = a;
  a = b;
  b = tmp;
}
  

