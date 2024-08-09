// malloc() and calloc() are for memory allocation at run time.
// new operator for creating a new object
// delete operator to delete the existing object.

int *p = new int;
float *q = new float;

delete []p;
delete []q;

//bad_alloc error is used instead of null pointer for handling allocation failure.



