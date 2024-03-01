//finding length
let myName = 'prc';
console.log(myName.length);

//escape character 
let anySentence = 'pussyboy "is the worst" ass';
console.log(anySentence);

//finding a string in string
const myBioData = 'I am a noob';
console.log(myBioData.indexOf("noob")); //displays the index number of the word

//search() method
let Data = myBioData.search("noob");
console.log(Data);

//slice method {gives from start to end}
var str = "Apple, Banana, Kiwi";
let res = str.slice(7)
console.log(res);

//substring() {ignores the last index element mentioned}
let res1 = str.substring(7);
console.log(res);

//replace() method
let replaceData = myBioData.replace('noob', 'boob');
console.log(replaceData);

//charAt() {for extracting string characters}
let str = 'hello pussy';
console.log(str.charAt(0));

//charCodeAt() method {for extracting the unicode}
var str = 'hello bitchass';
console.log(str.charCodeAt(0));

//property access {gives the index element of the number}
var str = 'lmao';
console.log(str[0]);

//concat()
let fName = "john";
let lName = "doe";
console.log(fName.concat(lName));

//split() {converting string to array}
var txt = 'a,b,c,d,e';
console.log(txt.split(","));
console.log(txt.split(" "));
console.log(txt.split(" | "));

