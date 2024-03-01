public class main{
	public static void main(String[] args){
		

		Timer timer = new Timer();

		TimerTask task = new TimerTask() {

			 public void run(){
			 	System.out.println("task is complete");

			 }
		};

		// timer.schedule(task,3000);
		Calendar date = Calendar.getInstance();
		date.set(Calendar.YEAR,2024);
		date.set(Calendar.MONTH,Calendar.JUNE);
		date.set(Calendar.DAY_OF_MONTH,20);
		date.set(Calendar.HOUR_OF_DAY,2020);
		date.set(Calendar.MINUTE,0);

		time.schedule(task, date.getTime());
		timer.scheduleAtFixedRate(task, 0, 1000); //every second executes.

	}
}