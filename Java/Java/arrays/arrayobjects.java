// import foodarrayObjects;

package arrays;
public class arrayobjects {
    
    public static void main(String[] args){
        
        
        foodarrayObjects food1 = new foodarrayObjects("pizza");
        foodarrayObjects food2 = new foodarrayObjects("burger");
        foodarrayObjects food3 = new foodarrayObjects("gamers"); 
        
        foodarrayObjects[] refrigerator = {food1, food2, food3};

        System.out.println(refrigerator[0].name);
    }
}
