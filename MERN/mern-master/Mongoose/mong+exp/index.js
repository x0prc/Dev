//boilerplate code for express
const express = require('express');
const app = express();
const path = require('path');


//integrating mongoose
const mongoose = require('mongoose')

mongoose.connect('mongodb://localhost:27017/farmStand')
    .then (()=> {
        console.log('mongo connection open')
    })
    .catch((err) => {
        console.log('mongo error')
        console.log(err)
    })

//boilerplate continued
app.set('views', path.join(__dirname, 'views'));
app.set('view engine', 'ejs');

//waiting for products
app.get('/products', async (req,res) => {
    const products = await Product.find({})
    res.render('products/index', {products})
})

//adding new products
app.get('/products/new', (req, res) => {
    res.render('products/new')
})

app.post('/products', async (req, res) => {
    const newProduct = new Product (req.body)
    await newProduct.save();
    console.log(newProduct)
    res.send('making your product')
})

app.listen(3000, () => {
    console.log('it works')
})

//updating products (ejs file with list of products)
app.get('/products/:id/edit', async (req, res) => {
    const prouct = await Product.findbyId(id);
    res.render('products/edit')
})

//deleting products
app.delete('/products/:id', async (req, res) => {
    const { id } = req.params;
    const deletedProduct = await Product.findbyIdAndDelete(id)
    res.redirect('/products')
})