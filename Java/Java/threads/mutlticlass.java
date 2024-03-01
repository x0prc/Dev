public class multiclass extends multithreading{


	public void run(){

		for(int i=10; i>0; i--){

			System.out.println(""+i);
			try{
				Thread.sleep(1000);
			} catch (InterruptedException e) {
				e.printStackTrace();

			}
		}

		System.out.println("finito")
	}
}