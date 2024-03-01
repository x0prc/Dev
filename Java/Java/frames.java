import javax.swing.JFrame;
import javax.swing.JLabel;
import java.awt.Color;


import javax.swing.ImageIcon;

public class frames {
    public static void main(String[] args){
        ImageIcon image = new ImageIcon("hehe.png");

        JFrame frame = new JFrame();
        frame.setTitle("gamerrrr");
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setResizable(false);
        frame.setSize(420,420);
        frame.setVisible(true);

        JLabel label = new JLabel();  
        label.setText("choder");
        label.setIcon(image);
        label.setHorizontalTextPosition(JLabel.CENTER);
        label.setVerticalTextPosition(JLabel.TOP);
        label.setForeground(new Color(123, 50, 250));


        frame.setIconImage(image.getImage());
        frame.getContentPane().setBackground(new Color(123, 50, 250));

       
}
}