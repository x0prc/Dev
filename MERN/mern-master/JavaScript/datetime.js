let currDate = new Date();
console.log(currDate);
console.log(new Date());
console.log(new Date().toLocaleString()); //local area
console.log(new Date().toString());

//Date.now() {displays milliseconds since Jan 1970}
console.log(Date.now());
var d = new Date(0);
console.log(d.toLocaleString());

//set custom dates {gives values in milliseconds}
console.log(currDate.setFullYear(2022));
console.log(currDate.setMonth(10));

//fetch individual time
const curTime = new Date();
console.log(curTime.getTime());
console.log(curTime.getHours());
console.log(curTime.getMinutes());
console.log(curTime.getSeconds());
console.log(curTime.getMilliseconds());

//set custom time
console.log(curTime.setTime());
console.log(curTime.setHours());
console.log(curTime.setMinutes());
console.log(curTime.setSeconds());
console.log(curTime.setMilliseconds());
