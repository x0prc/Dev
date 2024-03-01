package interfaces;

public class fish implements prey,predator {
    @Override
    public void hunt(){
        System.out.println("this fish is hunting.");
    }

    @Override
    public void flee(){
        System.out.println("boom boom");
    }
    
}
