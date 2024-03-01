// //if-else control statements.
var tomr = 'rain'; 
if(tomr == 'rain'){  
    console.log('take a raincoat');
}
else{
    console.log('dont take a raincoat')
}

//falsy values: 0, "", undefined, null, NaN, false

//conditional (ternary) operator
var age = 17; 
(age >= 18) ? "you can vote" : "you cant vote";


//switch case and break statements
var area = "triangle"; 
var PI = 3.14, l=5, b=4, r=3;
switch(area){
    case 'triangle':
        console.log ("area of triangle is" + (l*b)/2);
    break;
    case 'circle':
        console.log ("area of circle is" + PI*r*r);
    break;
}

//while loop statement (block scope)
var num=0;
while(num <= 10){
    console.log(num);
    num++;
}

//do while loop (condition is checked later)
var num = 10; 
do{
    console.log(num);
    num++
}while(num <= 10);

//for(initialiser; condition; iteration)
var num = 20; 
for(var num = 20; num <= 10; num++){
    console.log(num);
}

