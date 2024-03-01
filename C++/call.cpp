#include<iostream>

using namespace std;

int main()
{ 

    void swapPointer (int*v, int*h);{  //swap values of a in b and vice versa.

        //int temp = *v;     //void functions dont return anything. 
        //*v = *h; 
        //*h = temp; 
    }

    int main();{
        int v = 5; 
        int h = 6;  
        swapPointer (&v, &h);     //call by value. (swaps values of v and h)
        cout<<v<<endl<<h<<endl;
        return 0; 

    }

    int swapReferenceVar (int a, int b);{       //call by reference. takes reference addresses of a and b and changes values.

        int temp = a; 
        a = b; 
        b = temp; 

        return 0; 
    }

    int main();{
        int a = 5; 
        int b = 6;  
        swapReferenceVar (&a, &b);     //call by reference. (swaps values of a and b)
        cout<<a<<endl<<b<<endl;
        return 0; 

    }
    return 0; 
}   


