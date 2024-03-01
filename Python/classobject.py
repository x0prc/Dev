# classes are blueprints
# objects are things that are built

# var => attributes
# func => methods
class Movie : # this is a class
    def __init__(self,title,year,have_seen): # self is the key 
        self.title = title # these are objects
        self.year = year
        self.have_seen = have_seen

    def nice_print(self):
          print("Title: ", self.title) 
          print("Year of production: ", self.year)
          print("I have seen it: ", self.have_seen)

film_1 = Movie("Life of Brian", 1949, True)
film_2 = Movie("baller moment", 1924, False)

print(film_1.title, film_1.have_seen)
film_2.nice_print() 
