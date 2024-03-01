package polymorphism;
import java.util.Scanner;

public class dynamicpoly {
    // polymorphism = many shapes/forms.
    // dynamic = after compilation (during runtime).
    public static void main(String[] args){
            Scanner scanner = new Scanner(System.in);
            animal animal;
    
            System.out.println("what animal do yo want");
            System.out.println("(1=dog) or (2=cat): ");
            int choice = scanner.nextInt();
    
            if(choice==1){
                animal = new dog();
                // animal.speak();
            }
    
            else if(choice==2){
                animal = new cat();
                // animal.speak();
            }
    
            else{
                animal = new animal();
                System.out.println("invalid choice");
                // animal.speak();
            }
            System.out.println(animal);
            scanner.close();
    
        }
    }

