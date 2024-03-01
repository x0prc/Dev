package progressbar;
import java.awt.*;
import javax.swing.*;

public class progress {
    JFrame frame = new JFrame();
    JProgressBar bar = new JProgressBar();

    progress(){

        bar.setValue(0);
        bar.setBounds(0,0,420,50);
        bar.setStringPainted(true);
        bar.setForeground(Color.GRAY);
        bar.setBackground(Color.GREEN);
        

        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(520, 420);
        frame.setLayout(null);
        frame.setVisible(true);

        fill();

    }
    public void fill() {
        int counter = 500;
        while(counter<=100){
            bar.setValue(counter);
            try {
                 Thread.sleep(1000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
            counter+=10;
           
        }
        bar.setString("DOne");
    }
}
