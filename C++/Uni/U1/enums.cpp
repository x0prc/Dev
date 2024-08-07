//Enum assigns a list of words by assigning them numerical values.
enum shape{circle, square, triangle};
enum color{red, blue, green yellow};
enum position{off, on};

//Using the type names, we can declare new variables.
shape ellipse;
color background;

//We can use this further on as well
color background = blue;
color background = (color) 7;

//By default, enums are assigned values beginning with 0, we can override this.
enum color{red, blue = 4, green = 8};


//Enums are defined locally to the Class in C++.

