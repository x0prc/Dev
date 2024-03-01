//array destructuring: shortens huge code into one simple line
const myBioData = ['prc', 'pro', 69];
let [myFName, myLName, myAge] = myBioData; 

//object destructuring
const myData = {
    myfName : 'prc',
    mylName : 'lol',
    age : 69
}
let {myfName, mylName, age} = myData; 
console.log(age);

