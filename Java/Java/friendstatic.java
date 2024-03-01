public class friendstatic {
    String name;
    static int numberOfFriends;

    friendstatic(String name){
        this.name = name;
        numberOfFriends++;
    }
    
    static void displayFriends(){
        System.out.println("you have"+numberOfFriends+"friends");
    }
}
