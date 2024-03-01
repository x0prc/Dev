var myHomies = ['nigga1', 'nigga2', 'nigga3'];
//Index Numbers = 0         1         2
//first element: Lower Index/Boundary
//last element: Upper Index/Boundary

//traversal array
console.log(myHomies[1]);
console.log(myHomies.length); //to check the length of elements.
console.log(myHomies[myHomies.length - 1]); //to check total length.

//for loop 
for(var i=0; i<myFriends.length; i++ ){
    console.log(myFriends[i]);
}

//for in loop (returns index number)
for(let elements in myHomies){
    console.log(elements);
}

//for of loop (returns element content)
for(let elements of myHomies){
    console.log(elements);
}

//for each loop (returns index number, element and array group)
myHomies.forEach(function(element, index, array){
    console.log(element + index + array);
});


//Searching and Filtering
var result = myHomies.filter(word => word.length > 3);
console.log(result);


//CRUD = Create, Read, Update, Delete

//Array.prototype.push() {adds elements at the end}      
myHomies.push('nigga5', 'nigga6');
console.log(myHomies);

//Arrary.prototype.unshift() {adds elements in the front}
myHomies.unshift('nigga-1', 'nigga0');
console.log(myHomies);

//Array.prototype.pop() {removes the last element and returns it; changes the length of array}
console.log(myHomies.pop());

//Array.prototype.splice() {removes the mentioned index element and adds next at the end}
const newHomies = myHomies.splice(4,0,"pussyboy");

//Array.prototype.update() {deletes the old element and updates it}
const updateHomies = myHomies.splice(1,1, 'NIgga1')

//Array.prototype.map() {returns a new array of result} {chainable method}
const array1 = [1, 4, 9, 16, 25 ];
let newArr = array1.map((curElem, index, arr) => {
    return `Index no = ${index} ${curElem} ${arr}` 
})
console.log(newArr);

//Square Root
let arr = [25, 36, 49, 64, 81];
let arrSqr = arr.map((curElem) => {
    return Math.sqrt(curElem);
})
console.log(arrSqr);

//Array.prototype.reduce() {to find a single source array, current value, current index, source array}
let arr4 = [5,6,2];
let sum = arr.reduce((accumulator, curElem, index, arr4) => {
    return accumulator += curElem;
})

//creating flat arrays
let flatArr = arr.reduce((accum, currVal) => {
    return accum.concat(currVal);
})

