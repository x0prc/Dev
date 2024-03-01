class Person:
    def move(self):   #these are objects
        print("Moves 4 paces")
    def rest(self):
        print("Gains 4 health points")

class Doctor(Person):   #inherits all the things that a person has
    def heal(self): print('heals 10 health points') # can use pass if there is no function

class Fighter(Person):
    def fight(self):print("Do 10 health points of damage")
    def move(self):print("Moves 6 paces")
        
class Wizard(Doctor,Fighter): # can inherit from 2 classes as well.
    def cast_spell(self):print("Turns invisble")
    def heal(self):print("Heals 15 health points")

character1=Doctor()
character1.heal()