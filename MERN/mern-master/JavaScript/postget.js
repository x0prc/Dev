const express = require('express');
const app = express();

//to get the input from user for amount of tacos
app.get('/tacos', (req,res) => {
    res.send('get /tacos response')
})
//to submit the output about the amount of tacos
app.post('/tacos', (req,res) => {
    res.send('post /tacos response')
})

app.listen(3000, () => {
    console.log('on port 3K')
})