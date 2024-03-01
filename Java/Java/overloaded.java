public class overloaded {
    //multiple constructors within a class with the same namme, but have different parameters
    String bread;
    String sauce;
    String cheese;
    String topping;

    overloaded(String bread, String sauce, String cheese, String topping){

        overloaded pizza = new overloaded("thicc crust", "tomato", "mozerella", "pepperoni");
    
        System.out.println(pizza);

    }
}
