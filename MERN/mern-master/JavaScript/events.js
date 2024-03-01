//calling functions in 3 types.
const callingFunction = ()  => {
    alert('most common way of writing functions');  
}

const thirdWay = document.getElementById('thirdWay');
thirdWay.onclick = function() {
    alert('another most common way of writing functions')
}

const fourthWay = document.querySelector('fourth way');
fourthWay.addEventListener('click', () => {
    alert('this works too')
})

//event objects
const fourhWay = document.getElementById('fourhWay');
const checkEvent = () => {
    console.log(event);
    console.log(event.target);
    console.log(event.type);
}

//mouse event: mouseDown(), mouseUp(), mouseenter, mouseleave
//keyboard event: keyPress(), keyDown()
//input events: inputChange

const selectElement = () => {
    const inputChange = document.getElementById('ice');
    const iceCreams = document.getElementById('iceCreams');
    console.log(`${inputChange} and ${iceCreams}`);
}

//timing events
setTimeout
clearTimeout
setInterval
clearInterval

const showMyName = document.querySelector('#showMyName')
const btn = document.querySelector('#btn')

//top to bottom: capturing phase
//bottom to top: bubbling phase
//target phase: targeting phase