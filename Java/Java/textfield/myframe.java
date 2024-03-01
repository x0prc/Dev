package textfield;
import java.awt.event.ActionListener;
import java.awt.FlowLayout;
import java.awt.event.ActionEvent;

import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JTextField;
import java.awt.Dimension;

public class myframe extends JFrame implements ActionListener{
    
    myframe(){
        this.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        this.setLayout(new FlowLayout());
        
        JButton button = new JButton("Submit");
        JTextField textfield = new JTextField();
        textfield.setPreferredSize(new Dimension(250,30));

        this.add(button);
        this.add(textfield); 
        this.pack();
        this.setVisible(true);

    }
    
    @Override
    public void actionPerformed(ActionEvent e) {
        throw new UnsupportedOperationException("Unimplemented method 'actionPerformed'");
    }
    //same way checkboxes, radio buttons, comboboxes, sliders.
}
