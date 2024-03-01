import javax.swing.AbstractAction;
import javax.swing.Action;
import javax.swing.JFrame;
import javax.swing.JLabel;

import java.awt.Color;
import java.awt.event.ActionEvent;

public class game {

    JFrame frame;
    JLabel label;
    Action upAction;
    Action downAction;
    Action leftAction;
    Action rightAction;
    
    game(){

        frame = new JFrame("Keybindssss bitch");
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(420,320);
        frame.setLayout(null);

        label = new JLabel();
        label.setBackground(Color.RED);
        label.setOpaque(true);



        frame.setVisible(true);
        
    }

    public class upAction extends AbstractAction{
        public void actionPerformed(ActionEvent e ){
             
        }
    
    }
    public class leftAction extends AbstractAction{
        public void actionPerformed(ActionEvent e ){
             
        }
    }
    public class rightAction extends AbstractAction{
        public void actionPerformed(ActionEvent e ){
             
        }
    }
    public class downAction extends AbstractAction{
        public void actionPerformed(ActionEvent e ){
             
        }
    }

}

