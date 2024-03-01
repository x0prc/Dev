//asynchronous: waits for the mentioned durationed and completes other tasks side by side
//async always returns a promise
const sing = async() => {
    return 'welcome to the cum zone'
}


//try and catch are used to collect errors and either print or store on memory.
sing()
    .then(data => {
        console.log('promise resolved with', data)
    })
    .catch(err => {
        console.log('rejected')
        console.log(err)
    })

    
//login tool with async
const login = async (user, pass) => {
    if(!user || !pass) throw 'missing'
    if(pass === 'capybara') return 'hello'
    throw 'invalid'
}

login('rando', 'capybara')
.then(msg => {
    console.log('logged in')
    console.log(msg)
})
.catch(err => {
    console.log('error')
    console.log(err)
})

