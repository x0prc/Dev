//same method, multiple parameters.
public class overload {
    public static void main(String[] args){

    }

    static int add(int a, int b){
        System.out.println("overloaded");
        return a+b;
    }

    static int add(int a, int b, int c){
        System.out.println("number 2");
        return a+b+c;
    }

    static int add(int a, int b, int c, int d){
        System.out.println("number222");
        return a+b+c+d;
    }
}
