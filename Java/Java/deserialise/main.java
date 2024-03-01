import java.io.Serializable;

//steps to deserialise

public class main{

	public static void main(String[] args) throws IOException, ClassNotFoundException {


		User user = null;

		FileInputStream fileIn = new FileInputStream();

		ObjectInputStream in = new ObjectInputStream(fileIn);
		user = (User) in.readObject();

		in.close();
		fileIn.close();		


		System.out.println(user.name);
		System.out.println(user.password);
		user.sayHello();
		
	}
}