const express = require('express');
const app = express();
const session = require('express-session');

app.use(session({secret: 'bigassecret'}));

app.get('/viewcount', (req, res) => {
    if (req.session.count) {
        req.session.count += 1; 
    } else {
        req.session.count = 1;
    }
    
    res.send(`you have viewed this page ${req.session.count} times`)
})

const sessionConfig = {
    secret: 'massivesecret',
    resave: false, 
    saveUninitialized: true, 
    cookie: {
        expires: Date.now() + 1000 * 60 
    }
}

app.listen(3000, () => {
    console.log('we listenin')
})