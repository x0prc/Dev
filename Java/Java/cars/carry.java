package cars;
public class carry {
    private String make;
    private String model;
    private int year;

    carry(String make, String model, int year){
        this.setMake(make); 
        this.setModel(model);
        this.setYear(year);
    }
    carry(carry x){
        this.copy(x); // assigned values from different objects (Copying.)

    }

    public void setModel(String model){
        this.model = model;
    }
    public void setYear(int year){
        this.year = year;
    }
    public void setMake(String make){
        this.make = make;
    }
    public void copy(carry x){ 
        this.setMake(make); 
        this.setModel(model);
        this.setYear(year);

    }

}

