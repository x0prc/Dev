#include <stdio.h>

FILE *fptr;

char filename[] = "file2.dat";

fptr = fopen(filename, "w");

if(fptr == NULL) {

  printf("Error in File Creation");
  
  fprintf(fptr, "yoo\n");
}

fscanf(fptr, "%d %d", &a, &b);
