const flash = require('connect-flash')
const express = require('express')
app.use(flash());

app.get('/', (req, res) => {
    res.render('home')
});

req.flash('success', 'made a campground')
