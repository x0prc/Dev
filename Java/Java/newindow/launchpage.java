package newindow;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import javax.swing.JButton;
import javax.swing.JFrame;

public class launchpage implements ActionListener {
    JFrame frame = new JFrame();
    JButton button = new JButton(null, null);

    launchpage(){
        
        button.setBounds(100,120,150,50);
        button.setFocusable(false);
        
        frame.add(button);
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(520, 250);
        frame.setLayout(null);
        frame.setVisible(true);
    }

    public void actionPerformed(ActionEvent e){
        if(e.getSource()==button) {
            // newwindow window = new newwindow();
            
        } 
    }
}
