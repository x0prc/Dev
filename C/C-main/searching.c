#include <stdio.h>


// linear searching
  int linear_search(int a[], int size, int key)
  {
    int pos = 0;
    while((pos < size) && (a[pos] != key))
      pos++;

    if(pos < size)
      return pos;
    return -1;
  }

// binary searching
int bin_search (int x[], int size, int key)
{

  int L, R, mid;
  L = -1; R = size;

  while (L+1 != R)
  {
    mid = (L + R) / 2;
    if(x[mid] <= key)
      L = mid;

    else R = mid;
  }

  if (L >= 0 && x[L] == key) return L;
  else return -1;

}

// selection sort
// assume that some parts are sorted and others have to be sorted.
// select and swap the minimum values while moving towards the n-1 values till the array is sorted in the required order.

int selsort (int x[], int size)
{
  int k,m;
  for (k = 0; k < size-1; k++)
  {
    m = min_loc(x, k, size);
    temp = a[k];
    a[k] = a[m];
    a[m] = temp;
  }
}

// insertion sort
// putting the middle point in the place where it should be.
// half is sorted already. half is to be sorted.
void insertSort (int list[], int size)
{
  int i, j;

  for(i = 1; i < size; i++)
  {
    item = list[i];
    for(j = i-1; (j>=0)&&(list[j] > i); j--)
      list[j+1] = list[j];
    list[j+1] = item;
  }
}

// bubble sort
// heaviest quantity rests in the bottom.
// lightest elements go up.
void swap(int *x, int *y)
  {
    int tmp = *x;
    *x = *y;
    *y = tmp;

  }

void bubble_sort(int x[], int n)
{
  
  int i,j;

  for(i = n-1; i > 0; i--)
    for (j = 0; j < i; j++)
      if(x [j] > x [j + 1] swap(&x[j], &x[j+1]));
}

//lexicographic sort
// compare letter by letter
