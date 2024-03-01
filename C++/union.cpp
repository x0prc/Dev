#include <iostream>
using namespace std;
int main(){	
union money {   //union. executes only one statement demanded. helps in memory management,.
    int rice;
    char car;           
    float rupees;
   };
return 0; 
}