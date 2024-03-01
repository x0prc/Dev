#include <iostream>
#include <math.h>
using namespace std;

int factorial(int n){	//  n! = n*(n-1)! [factorial definition]
    if (n<=1){
        return 1;
    }
return n * factorial(n-1); 
}
int main(){
    int a; 
    cout<<"enter a number"<<endl;
    cin>>a;
    cout<<"the factorial of " << a << " is "<<factorial(a);
}

int fib (int n){
    if(n<2){
        return 1;
    }
    return fib(n-2) + fib(n-1);    //recursive function for fibonacci sequence
}   