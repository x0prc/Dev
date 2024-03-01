package newindow;

import javax.swing.JFrame;
import javax.swing.JLabel;
import java.awt.Font;

public class newwindow {
    JFrame frame = new JFrame();
    JLabel label = new JLabel();

    newwindow(){

        label.setBounds(0,0,100,150);
        label.setFont(new Font(null, Font.PLAIN,25));

        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(520, 250);
        frame.setLayout(null);
        frame.setVisible(true);
    }
}
