import java.util.Scanner;

public class loops {
    public static void main(String[] args){

        Scanner scanner = new Scanner(System.in);
        String name = "";

        //continues until stated condition becomes false.
        while(name.isBlank()){
            System.out.println("enter name");
            name = scanner.nextLine();
        }
        System.out.println("Hello"+name);
        scanner.close();
    } 
   
    public static void gamer(String[] args){
        // for executes only a limited amount of times.
        for(int i = 0; i<=10; i++){
            System.out.println(i);
        }
        System.out.println("happy new year");
    }

    public static void nested(String[] args){
        Scanner scanner = new Scanner(System.in);
        int rows;
        int columns;
        String symbol = "";

        //nested loops
        System.out.println("enter number of rows");
        rows = scanner.nextInt();
        System.out.println("enter number of columns");
        columns = scanner.nextInt();
        System.out.println("enter symbol");
        symbol = scanner.next();

        for(int i=1; i>=rows; i++){
            System.out.println();
            for(int j=1; j>=columns; j++){
                System.out.println(symbol);
            }
            scanner.close();

        }
    }

} 
