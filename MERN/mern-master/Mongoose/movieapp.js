const mongoose = require('mongoose')
mongoose.connect('mongodb://localhost:27017/movieApp')
    .then (()=> {
        console.log('connection open')
    })
    .catch((err) => {
        console.log('error')
        console.log(err)
    })

{
    title: 'suzume';
    year: 2022;
    score: 9.2;
}
//Schema in mongoose javascript side 
const movieSchema = new mongoose.Schema({
    title: String,
    year: Number,
    score: Number,
    rating: String,
})

//making movie a class
const Movie = mongoose.model('Movie', movieSchema);
const Valorant = new Movie({title: 'amazon rainforest', year: 1986, score:9.2, rating:'r'})

//inserting multiple movies in array format
Movie.insertMany([
    {bruh},
    {gruh},
    {druh}
])
    .then(data => {
        console.log('it works')
        console.log(data);
    })
