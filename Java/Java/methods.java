public class methods {
    public static void method(String[] args){
        //method executes when code is called.
        String name = "bro";
        int age = 21;
        
        hello(name,age);

    }
    static void hello(String name, int age){ //use void if not returning.
        System.out.println("yoyo" +age);
        System.out.println("hello" +name);
    }

     
    static int add(int x, int y){
        int z = x + y;
        return z;
    }
}
