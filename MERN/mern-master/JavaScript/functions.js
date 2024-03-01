//function definition is also called as function declaration or function statement.
function sum(){
    var a = 10, b = 20; 
    var total = a+b;
}

//calling functions
console.log(sum());

//function arguements
sum(20,30)
sum(50,50)

//function expressions
function sum(a,b){               //anonymous fuctions
    return total = a+b;
}
var funExp = sum(5,14);
console.log('sum is' + funExp);

//block scope, cannot be used outside func.
function biodata(){
    let myName = "test subject";
    const yourName = "gg";
}

//default parameters
function mult(a,b){
    return a*b;
}
console.log(mult(5));

//fat arrow function
const sum  = () => `the sum of the two numbers is ${(a=5)+(b=6)}`;
console.log((sum));

//higher order function : passes an arguement inside
calculator(2,4,subs)

//callback function : is itself an arguement
const add = (a,b) => {
    return a+b;
}
