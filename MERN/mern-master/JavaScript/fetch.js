//for fetching the api
fetch('https://swapi.dev/api/people/1')

.then(res => {
    console.log('resolve', res)
    res.json().then((data) => console.log('json done', data));
})
.catch(e => {
   console.log('error', e) 
});

const loadStarWarsPeople = async () => {
    const res = await fetch('https://swapi.dev/api/people/1')
    const data = await res.json();
    console.log(data);
}; 

loadStarWarsPeople();