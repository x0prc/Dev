const Product = require('/Users/prclol/Documents/GitHub/mern/Mongoose/mong+exp/ducts/product');

const mongoose = require('mongoose')


mongoose.connect('mongodb://localhost:27017/farmStand')
    .then (()=> {
        console.log('mongo connection open')
    })
    .catch((err) => {
        console.log('mongo error')
        console.log(err)
    })

const p = new Product({
    name: 'Ruby', 
    price: 1.99, 
    category: 'fruit'
})

//adding seed data to the database
const seedProducts = [
    {
        name: 'grapes',
        age: '5 days'
    }
]

Product.insertMany(seedProducts)
.then(res => {
    console.log('works')
} )
.catch(res => {
    console.log('doesnt work')
})