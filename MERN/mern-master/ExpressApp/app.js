const express = require('express');
const app = express();
const engine = require('ejs-mate');
const mongoose = require('mongoose')
const Campground = require('./models/campground')

mongoose.connect('mongodb://localhost:27017/yelp-camp', {
   useNewUrlParser: true,
   useCreateIndex: true,
   useUnifiedTopology: true 
});

const db = mongoose.connection;
db.on('error', console.error.bind(console,'connection error'));
db.once('open', () => {
    console.log('database connected');
});

app.engine('ejs-mate')
app.set('view engine', 'ejs');
// app.set('views', path.join(__dirname, 'D:\mern\ExpressApp\views'))

app.get('/', (req,res) => {
    res.send('hello')
})

app.get('/campgrounds', async (req,res) => {
    const campgrounds = await Campground.find({});
    res.render('campgrounds/index')
})
app.listen(3000, () => {
    console.log('connected to port 3000')
})