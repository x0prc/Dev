#include <iostream>
using namespace std;
int main(){	
/* operators: 
    comparison: >, <, ==, !=, >=, <=
    assignment: char, int, bool
    arithmetic: +, -, /, *, %, ++a, --a, a++, a--
    logical: ||, &&, ! [and, or, not]
*/
    cout<<"Hello world"<<endl;
    
    float x = 455; 
    float & y = x; //reference variable
    cout<<x<<endl;
    cout<<y<<endl; 

return 0; 
}