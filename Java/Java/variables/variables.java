package variables;
import java.util.Scanner;
public class variables {
    public static void main(String[] args){
        int x = 123; //initialisation
        double y = 2.31;
        boolean z = true;
        char symbol = '@';
        String name = "boii";

        String a = "baller";
        String b = "gamer";
        String temp; //temporary values.
        
        temp = a;
        a=b;
        b=temp; //swapping two variables.


        Scanner scanner = new Scanner(System.in); //scanners working in java.
        System.out.println("what is your name");
        String boi = scanner.nextLine();
        System.out.println("whats your age");
        int age = scanner.nextInt();


        System.out.println("ghello" +name); 
        System.out.println("you are " +age+"years old");
        System.out.println(y);
        System.out.println(z);
        System.out.println(symbol);
        System.out.println(boi);

        System.out.println("My number is: "+x); //for taking input

        scanner.close();
        }
}
// boolean : 1 bit
// int : 4 bytes
// long : 8 bytes
// float : 4 bytes
// double : 8 bytes
// char : 2 bytes
// string : varies
