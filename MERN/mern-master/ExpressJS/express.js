//using express npm
const express = require("express");
const app = express()

app.use((req, res) => {
    console.log('we got a new request')
    res.send('hello we got the request')
})
app.listen(8080, () => {
    console.log('listening on port 8080')
})

//getting routing pages in express
app.get('*', (req,res) => {
    res.send(`i dont know`)
})

app.get('/r/subreddit/:postId', (req, res) => {
    const { subreddit, postId } = req.params;
    res.send('balls')
})

//post routing pages in express
app.post('/cats', (req, res) => {
    res.send('post request please')
})
