//representational state transfer allows functioning like redirect, link, submit to database
const express = require('express');
const uuid = require('uuid');
const {v4 : uuidv4} = require('uuid');
const app = express();
const path = require('path');

app.use(express.urlencoded({ extended: true }))
app.use(express.json())
app.set('views', path.join(__dirname, 'views'))
app.set('view engine', 'ejs')


const comments = [
    {
        id: uuid(), 
        username: 'prc',
        comment: 'bussin'
    }
]
// INDEX : /comments : list of comments
// NEW : /comments/new : new comment 
// CREATE : /comments (POST) : creates a new comment
// PATCH: /comments/:id : change a comment
// EDIT : /comments/:id/edit : edit body of the text.
// DESTROY : /comments/:id/delete : delete body or comment

app.get('/comments/new', (req,res) => {
    res.render('comments/new')
})
app.get('/tacos', (req,res) => {
    res.send('get /tacos response')
})
app.post('/comments', (req,res) => {
    const { username, comment } = req.body; 
    comments.push({username, comment, id: uuid() })
    //express.redirect() usage: 
    res.redirect('/comments');
})

app.get('/comments/:id', (req,res) => {
    const { id } = req.params;
    const comment = comments.find(c => c.id === id);
    res.render('comments/show', { comment })
})

app.listen(3000, () => {
    console.log('on port 3K')
})