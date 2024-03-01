//embedding is used to embed js code in html/ejs files.
const express = require('express');
const app = express();
const path = require('path');

//specifying a directory using static
app.use(express.static(path.join(___dirname, 'public')))
app.set('view engine', 'ejs');

//to work while we are outside the directory where it is placed
app.set('views', path.join(___dirname, '/views'))

//render used for linking ejs file
app.get('/', (req, res) => {
    res.render('home')
})

app.get('/r/:subreddit', (req, res) => {
    const { subreddit } = req.params;
    res.render('subreddit', { subreddit });
})
app.get('/rand', (req, res)=> {
    const num = Math.floor(Math.random() * 10) + 1;
    res.render('random', { rand: num })
})

//to listen on the port 3000
app.listen(3000, () => {
    console.log('listening on port 3000')
})


