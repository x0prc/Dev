#include <iostream>
#include <iomanip>
using namespace std;
int main(){	
const int balls = 4; //constant, cannot change value later
    cout<<balls<<endl;

    int s=3, p=25, q=567; 
    cout<<setw(4)<<s<<endl;
    cout<<setw(4)<<p<<endl;
    cout<<setw(4)<<q<<endl; //sets width to 4 spaces. (setw is a manipulator)
return 0; 
}