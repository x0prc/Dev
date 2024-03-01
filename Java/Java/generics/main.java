package generics;

import java.util.ArrayList;

public class main {
    public static void Main(String[] args){

        MyIntegerClass <Integer> myInt = new MyIntegerClass<>(1);
        MyDoubleClass <Double> myDouble = new MyDoubleClass<>(2.12);
        MyCharacterClass <Character> myChar = new MyCharacterClass<>('@');
        MyStringClass <String> myString = new MyStringClass<>("heeeeee");

        ArrayList<String> myFriends = new ArrayList<>();

        System.out.println(myInt.getValue());
        System.out.println(myDouble.getValue());
        System.out.println(myString.getValue());
        System.out.println(myChar.getValue());
    }

}