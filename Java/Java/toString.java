import cars.car;

public class toString {

    public static void main(String[] args){
      car car = new car();
        //using explicitly.
        System.out.println(car.toString()); //prints the address of the string
        //using impplicitly.
        System.out.println(car);
        
    }
}

