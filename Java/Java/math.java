import java.util.Scanner;
public class math {
    public static void main(String[] args){
        double x = 2.31;
        double y = -10;

        double z = Math.max(x, y);
        double f = Math.min(x,y);
        double s = Math.abs(y);
        double q = Math.sqrt(y);
        double r = Math.round(x);
        double c = Math.ceil(x);

        System.out.println(z);
        System.out.println(f);
        System.out.println(s); 
        System.out.println(q);
        System.out.println(r);
        System.out.println(c);
    }

    public static void triangle(String[] args){

        double x;  //finding hypotenuse with the angles of triangle.
        double y;
        double z;

        Scanner scanner = new Scanner(System.in);
        
        System.out.println("Enter side x:");
        x = scanner.nextDouble();
        System.out.println("Enter side y:");
        y = scanner.nextDouble();

        z = Math.sqrt((x*x)+(y*y));

        System.out.println("the hypotenuse is"+z);
        scanner.close();
    }
}
