import java.util.Scanner;
public class operators {
    public static void main(String[] args){
        
        int temp = 23;
        
        // AND logical operator (&) (both must be true.)
        if(temp>30) {
            System.out.println("it is hot outside");
        }
        else if(temp>=20 && temp<=30){ 
            System.out.println("warm outside");
        }
        else{
            System.out.println("baller moment");
        }
        
        Scanner scanner = new Scanner(System.in);
        System.out.println("gamer playing rn. press q or Q");
        String response = scanner.next();
        
        // OR logical operator || (either should be true)
        if(response.equals("q") || response.equals("Q")){
            System.out.println("quit it");
        }
        else{
            System.out.println("pew pew boii");
        }
        scanner.close();

        // NOT logical operator reverses boolean value of a condition. 
          if(!response.equals("q") && !response.equals("Q")){
            System.out.println("quit it");
        }
        else{
            System.out.println("pew pew boii");
        }
        scanner.close();
    }
}
