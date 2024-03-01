package arrays;
public class arrays {
    public static void main(String[] args){
        // String[] cars = {"Camaro", "Corvette", "tesla"};
        // cars[0] = "Mustand";

        //2D Array
        String[][] cars = {
            {"camao", "corvy","silver"},
            {""}
        };
       

        for(int i=0; i<cars.length; i++){
            System.out.println();
            for(int j=0; j<cars[i].length; j++){
                System.out.println(cars[i][j]+"");
            }
        }

        

        System.out.println(cars[0]);
    }
} 
