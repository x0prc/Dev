public class printf {
    public static void main(String[] args){
        System.out.printf("baller %d", 123); //%d -> decimal adding to position.

        System.out.printf("%b");
        // conversion characters :
        // %b = boolean 
        // %c = char
        // %s = string
        // %d = decimal
        // %f = double

        //width => minimum characters in output => 10
        System.out.printf("hellovffddsssdfffffdfdsswwww2w %10s");

        //precision => number of digits (in this case 2) of precision when outputting
        System.out.printf("gamer is a %.2f");

        //flags
        // - : left justify
        // + : out a plus or minus
        // 0 : numeric values turn 0
        // , : comma grouping separator numbers > 1000
        System.err.printf("gamer asdasdasdsadasd %,f");
        
 
    }
}
