package files;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

public class filewriter{
    public static void main(String[] args){
        try{
            FileWriter writer = new FileWriter("poem.txt");
            writer.write("roses are red");
            writer.append("\n(A poem by gamer)");
            writer.close();
        }
        catch(IOException e){
            e.printStackTrace();
        }
        File file = new File("poem.txt");
        System.out.println(file);
    }
}