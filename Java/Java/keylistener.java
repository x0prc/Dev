import java.awt.event.*;
import javax.swing.*;

public class keylistener extends JFrame implements KeyListener{
    
    JLabel label;

    keylistener(){
        this.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        this.setSize(500,500);
        this.setLayout(null);
        this.addKeyListener(this);
        this.setVisible(true);

    }
    public void keyTyped(KeyEvent e){
        // invoked when a key is typed.
        switch(e.getKeyChar()){
        case 'a' : label.setLocation(label.getX()-1, label.getY());
        }
    }
    public void keyPressed(KeyEvent e){
        // invoked when a key is pressed down, stops when released.
    }
    public void keyReleased(KeyEvent e){
        // invokes when key is released.
        System.out.println("you released key code:" + e.getKeyCode());
    }

    //works the same way for mousePressed, mouseReleased, mouseEntered, mouseExited.
}