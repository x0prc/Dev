import javax.swing.JOptionPane;

public class gui {

    public static void main(String[] args){

        String name = JOptionPane.showInputDialog("enter your name");
        JOptionPane.showMessageDialog(null, "Hello "+name);

        int age = Integer.parseInt(JOptionPane.showInputDialog("enter your age"));
        JOptionPane.showMessageDialog(null, "You are "+age+"years old");
    
        double height = Double.parseDouble(JOptionPane.showInputDialog("enter your height"));
        JOptionPane.showMessageDialog(null, "enter your height"+height);



    }
    
}
