const express = require('express');
const app = express();
const path = require('path');
const mongoose = require('method-override')
const AppError = require('./apperror')

//should be enclosed in every route/arguement
app.use((err, req, res, next) => {
    const { status = 500, message = 'wrong' } = err;
    res.status(status).send(message);
})

//can be used as a whole function wrapped around in async error handlers
function wrapAsync(fn) {
    return function(req,res, next){
        fn(req, res, next).catch(e => next(e))
    }
}

//printing mongoose errors in names 
const handleValidationErr = err => {
    console.log(err)
    return err;
}
app.use((err, req, res, next) => {
    console.log(err.name);
    if(err.name === 'validation error') err = handleValidationErr(err)
    next(err);
})