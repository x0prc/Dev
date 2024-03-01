# include <stdio.h>

void message( ) ; /* function prototype declaration */ 

void message( ) /* function definition */ 
{
	printf ( "Smile, and the world smiles with you...\n" ) ; 
}

int main( )
{
	message( ) ; /* function call */
	printf ( "Cry, and you stop the monotony!\n" ) ; 
	return 0 ;
}

# include <stdio.h>
float power ( float, int );

int new( )
{
  float x, pow;
  int y;
  printf ( "\nEnter two numbers: " ); 
  scanf ( "%f %d", &x, &y );
  
  pow = power ( x , y );
  printf( "%f to the power %d = %f\n", x, y, pow ); 
  return 0;
}

float power ( float x, int y ) 
{
  int i;
  float p=1;
  for ( i = 1 ; i <= y ; i++ ) 
    p=p*x;
  return ( p ); 

}
// test code

