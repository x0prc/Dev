//constructor here helps in assigning name and age values
class Pet {
    constructor(name, age) {
        this.name = name; 
        this.age = age; 
    }
    eat() {
        return `${this.name} is eating`;
    } 
}

class Cat extends Pet{}
class Dog extends Pet{}