public class threads{

	public static void main(String[] args){ //this is a thread.


		System.out.println(Thread.activeCount());

		Thread.currentThread().setName("bruhhhhhh"); 
		
		Thread.currentThread().setPriority(10);
		System.out.println(Thread.currentThread().getPriority());
		
		System.out.println(Thread.currentThread().getName());

		System.out.println(Thread.currentThread().isAlive());

		for(int i=2; i>0; i--){
			System.out.println(i);
			Thread.sleep(1000);
		}

		System.out.println("You are done");

//------------------------------------------------------------------

		MyThread thread2 = new MyThread();
		System.out.println(thread2.isAlive());
		thread2.start();

		thread2.isDaemon(); //low level bg process (daemon).

	}
}