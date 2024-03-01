const express = require('express');
const app = express();
const User = require('./models/user');
const mongoose = require('mongoose');
const bcrypt = require('bcrypt');
const session = require('express-session');


mongoose.connect('mongodb://localhost:27017/loginDemo')
.then(() => {
    console.log('connection open')
})
.catch(err => {
    console.log('connection error')
    console.log(err)
})

app.set('view engine', 'ejs');
app.set('views', 'views');

app.use(express.urlencoded({extended: true}));  
app.use(session({secret: 'not a good secret'}));

const requireLogin = (req, res ,next) => {
    if (!req.session.user_id) {
        return res.redirect('/login')
    } next();
} 
app.get('/', (req, res) => {
    res.render('register')
})
//registering on the website
app.get('/register', (req, res) => {
    res.render('register')
})

app.post('/register', async (req, res) => {
    const { password, username } = req.body;
    const hash = await bcrypt.hash(password, 12);  
    const user = new User({
        username,
        password: hash
    })
    await user.save();
    res.redirect('/');
})

//logging in on the website with password hash comparison
app.get('/login', (req, res) => {
    res.render('login')
})
app.post('/login', async (req, res) => {
    const {username, password} = req.body;
    const foundUser = await User.findAndValidate(username, password)  
    if (foundPassword) {
        res.redirect('/secret')
    }
    else {
        res.redirect('/login')
    }
})

//logging out of the website 
app.post('/logout', (req, res) => {
    req.session.user_d = null;
    req.session.destroy(); //logs out after session ends
    res.redirect('/login')
})

//using middleware to login
app.get('/secret',requireLogin, (req, res) => {
    res.render('secret')
})

app.listen(3000, () => {
    console.log('o ris it /')
})
