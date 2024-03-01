//faster and smaller than fetch
axios.get('https://swapi.dev/api/people/1')
.then(res => {
    console.log('response:', res)
})


//specified the type of header to fetch only that part. 
const getDadJoke = async() => {
    const config = { headers: { Accept: 'application/json'}}
    const res = await axios.get('https://icanhazdadjoke.com')
    console.log(res.data.joke)
}

