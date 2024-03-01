package generics;
public class generics {
    
    public static void main(String[] args){

        Integer[] intArray = {1,2,3,4,5};
        Double[] doubleArray = {3.3,1.3,4.4,5.3};
        Character[] charArray = {'H','E','L','L'};
        String[] stringArray = {"BBBBB","ASDADASD","WASDWASDWASD"};

        displayArray(intArray);
        displayArray(doubleArray);
        displayArray(charArray);
        displayArray(stringArray);

        System.out.println(getFirst(intArray));
        System.out.println(getFirst(doubleArray));
        System.out.println(getFirst(charArray));
        System.out.println(getFirst(stringArray));

        
    }
    
    //thing is a generic method. no need for individual method for each and every datatype.
    public static <Thing> void displayArray(Thing[] array){

        for(Thing x : array){
            System.out.println(x+"");
        }
        System.out.println();

    }
    public static <Thing> Thing getFirst(Thing[] array){
        return array[0];
    }
 
    
}
