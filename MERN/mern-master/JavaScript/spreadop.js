//spread operator: add data with ... later on 

const colors = ['red', 'green', 'blue', 'yellow']; 
const myColors = ['red', 'green', 'blue', 'yellow', 'black'];

const myFavColors = [ ...colors, 'yellow', 'black'];
console.log(myFavColors);

