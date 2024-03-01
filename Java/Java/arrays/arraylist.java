package arrays;
import java.util.ArrayList;

public class arraylist {
    public static void main(String [] args){

        ArrayList<ArrayList<String>> groceryList = new ArrayList<>();

        ArrayList<String> food = new ArrayList<String>();
        food.add("pizze");
        food.add("hambyrger");
        food.add("hotdog");
        
        food.set(0, "sushi");
        food.remove(2);
        food.clear();

        ArrayList<String> bakeryList = new ArrayList<String>();
        bakeryList.add("pasta");
        bakeryList.add("garlic bread");
        bakeryList.add("donuts");
        System.out.println(bakeryList.get(0));

        
        groceryList.add(bakeryList); //list of lists
        groceryList.add(food);

        for(int i=0; i<food.size(); i++){
            System.out.println(food.get(0));
        }

    }
}
