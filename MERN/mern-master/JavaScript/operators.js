var myName = 'prc';
console.log(myName);

//typeof operator displays the type of data input.
console.log(typeof(myName));

//boolean
var iAmPrc = true;
console.log(iAmPrc);

//adding numbers with strings
console.log(9 + "5");

//adding strings with strings
console.log("Java" + "Script");

//subtracting strings from strings : NaN (Not a number)
console.log("prc" - "prc");

//adding true and true boolean conditions
console.log(true + true);

//adding true and false boolean conditions
console.log(true + false);

//assigning null values.
var iAmChad = null;
console.log(iAmChad);

//assigning undefined values.
var iAmGigaChad;
console.log(iAmGigaChad);

//NaN
var myNumber = 9876543210;
var myAdd = "okay";
console.log(isNaN(myNumber));
console.log(isNaN(myAdd));

//arithmetic operators: +, -, /, *, %, **
console.log(27%4);


//postfix increment operator.
var num = 15; 
var newNum = num++;
console.log(num);
console.log(newNum);

//prefix increment operator.
var nnum = 15; 
var nigaNum = ++nnum + 5; 
console.log(nnum);
console.log(nigaNum);

//comparison operators
var a = 30;
var b = 10;
console.log(a==b); //equal
console.log(a!=b); //not equal
console.log(a>b); //greater than
console.log(a<b); //less than
console.log(a>=b); //greater than equal to
console.log(a<=b); //less than equal to

//logical operators
var c = 30;
var d = -20;
console.log(c > d && d > 0); //only outputs when all true.
console.log(c > d || b>0 || b>0 ); //either true or false inputs.
console.log(!(a>0) || (b<0)); //not operator. gives vice versa.

//string operators
console.log("hello " + "world"); // adds two strings.