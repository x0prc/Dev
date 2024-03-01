#include <stdio.h>

// strcpy, strlen, strtcat, strcmp
// copy, length, concatenation, comparison

int main()
{
  int i;

  char ch_arr[3][10] = {"spike", "tom", "jerry"};
  printf("1st way \n\n");

  for(i = 0; i < 3; i++)
  {
    printf("string = %s\t address = %u\n", ch_arr + i, ch_arr + i);
  }

  return 0;

}

