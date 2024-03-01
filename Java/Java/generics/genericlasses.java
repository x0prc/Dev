package generics;
public class genericlasses <Thing>{
    Thing x;

    genericlasses(Thing x){
        this.x = x;
    }
    public Thing getValue(){
        return x;
    }
}