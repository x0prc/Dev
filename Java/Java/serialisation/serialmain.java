import java.io.Serializable;



public class main{

	public static void main(String[] args) throws IOException{

		User user = new User();

		user.name = "balllll";
		user.password = "weee";

		fileOutputStream fileOut = new FileOutputStream("Userinfo.ser");
		ObjectOutputStream out = new ObjectOutputStream(fileOut);

		out.writeObject(user);
		out.close();
		fileOut.close();

		System.out.println("saved");

		

	}
}