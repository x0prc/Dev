#include <iostream>
using namespace std;

inline int product (int a, int b){
    return a*b;   //returns the product of a and b multiple times. (not recommended for programs.)
}

int product (int a, int v){
    static int c = 0;  //only executes once. 
    c = c + 1;  //continues from here further.
    return a*v+c;
}

float moneyReceived(int currentMoney, float factor = 1.04){
    return currentMoney*factor;    //default arguements.

}
