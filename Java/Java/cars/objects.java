package cars;
//calling from another file/class (car.java in this case)

public class objects {
    public static void main(String[] args){
        
        car myCar = new car();

        System.out.println(myCar.model);
        System.out.println(myCar.model);

        myCar.drive();
        myCar.brake();

    }
}
