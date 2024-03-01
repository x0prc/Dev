public class statick {
    public static void main(String[] args){
        // class owns the static member.
        friendstatic friend1 = new friendstatic("gang");
        friendstatic friend2 = new friendstatic("baller");

        friendstatic.displayFriends();

        // Math math = new Math(); //static modifiers.
        // Math.round(a);

        //name of static owner and then name of static member.
        System.out.println(friendstatic.numberOfFriends);
        System.out.println(friend1);
        System.out.println(friend2);

    }
    
}
