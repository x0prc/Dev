const express = require('express')
const app = express();
const morgan = require('morgan'); 
const AppError = require('./apperror')


//morgan displays requests in terminal
app.use(morgan('common'))
//next calls the matching middleware (as the name says next)
app.use((req, res, next) => {
    console.log('this is my first middleware')
    next();
})

app.use((req, res, next) => {
    console.log('hey')
    return next();
})

app.get('/error', (req, res) => {
    chicken.fly()
})

//authentication demo
app.use((req, res, next) => {
    const { password } = req.query;
    if (password === 'yomama'){
        next();
    }
    throw new AppError('password required', 401)     
})

app.get('/', (req, res) => { 
    res.send('homepage')
    })

//custom error handler
app.use((err, req, res, next) => {
    const { status = 500 } = err
    res.status(status).send('errorrr') 
})

app.listen(3000, () => {
    console.log('running on localhost')
    })
