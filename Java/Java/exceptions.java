import java.util.InputMismatchException;
import java.util.Scanner;


public class exceptions {
    public static void main(String[] args){
    Scanner scanner = new Scanner(System.in);

        try{

            System.out.println("whole number to divide:");
            int x = scanner.nextInt();

            System.out.println("enter a whole number to divide by: ");
            int y = scanner.nextInt();

            int z = x/y;

            System.out.println("result" + z);

        }
        catch(ArithmeticException e){
            System.out.println("you cant divide by 0 dumbass.");
        }
        catch(InputMismatchException e){
            System.out.println("enter a whole number oh my fucking god");
        }
        catch(Exception e){
            System.out.println("something went wrong.");
        }
        finally{
            scanner.close(); 
            System.out.println("this will always print");
            
        }
    }    
    
}
