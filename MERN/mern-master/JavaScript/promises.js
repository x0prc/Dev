//eventual completion or failure of asynchronous operation
//better than callback hell.
debugger

//format for creating promises.
const jokes = document.querySelector('#joke');
const jokeBtn = document.querySelector('#jokeBtn');

const generateJokes = () => {
    fetch('https://icanhazdadjoke.com')
    .then((res) => {
        console.log(res.json());
    })
    .catch((error) => {
        console.log('error');
    })
}
jokeBtn.addEventListener('click', generateJokes);
generateJokes();

//another format for creating promises 
let myPromise1 = new Promise((resolve,reject) => {
    setTimeout(() => {
      resolve("Promise 1 resolved")
    },6000)})
let myPromise2 = new Promise((resolve,reject) => {
    setTimeout(() => {
      resolve("Promise 2 resolved")
    },3000)})
  myPromise1.then((successMessage) => {
    console.log("From Callback " + successMessage)
  })
  myPromise2.then((successMessage) => {
  console.log("From Callback " + successMessage)
})