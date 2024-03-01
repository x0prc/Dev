import java.io.File;
import java.io.IOException;
import java.util.Scanner;
import javax.sound.sampled.*;

public class audio{
    public static void main(String[] args) throws UnsupportedAudioFileException, IOException{
        try {
        
        Scanner scanner = new Scanner(System.in);

        File file = new File("mgk.wav");
        AudioInputStream audioStream = AudioSystem.getAudioInputStream(file);
        Clip clip = AudioSystem.getClip();
        clip.open(audioStream);

        String response = "";

        while(!response.equals("Q")){
            System.out.println("P, S, R, Q");
            System.out.println("enter choice.");
        }
            response = scanner.next();
            response = response.toUpperCase();

        switch(response){
            case ("P"): clip.start();
            break;
            case ("S"): clip.stop();
            break;
            case ("R"): clip.setMicrosecondPosition(0);
            break;
            default: System.out.println("not valid response.");
        }
        scanner.close();

        } 
        catch (Exception e) {
            // TODO: handle exception
        }
       

       

    }
}