const express = require('express');
const app = express(); 
const cookieParser = require('cookie-parser')
app.use(cookieParser('mysecret'));

app.get('/greet', (req, res) => {
    console.log(req.cookie)
    res.send('hey there')
})

app.get('/setname', (req, res) => {
    res.cookie('name', 'prc');
    res.send('sent a cookie')
})

// /signing a cookie with cookie parser
app.get('/getsignedcookie', (req, res) => {
    res.cookie('fruit', 'grape', { signed: true})
    res.send('ok signed')
})
app.listen(3000, () => {
    console.log('working')
})