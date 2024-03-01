#include <iostream>
using namespace std;

int sum(int a, int b){
    cout<<"2 args"<<endl;
    return a+b;
}

int sum(int a, int b, int c){
    cout<<"3 args"<<endl;                       //multiple functions can be used with same name.   
    return a+b+c;
}

int main(){
    cout<<"sum is"<<sum(3,6)<<endl;
    cout<<"another sum is"<<sum(3,5,9)<<endl;
    return 0;                      
}