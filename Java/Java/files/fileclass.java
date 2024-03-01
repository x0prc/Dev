package files;
import java.io.File;

public class fileclass {
    public static void main(String[] args){
        File file = new File("secret_message.txt");

        if(file.exists()){
            System.out.println("yeahhh babbyyy");
            System.out.println(file.getPath());
            System.out.println(file.getAbsolutePath());
            file.delete();
        }
        else{
            System.out.println("it aint dere braah");
        }
    }
    
}
