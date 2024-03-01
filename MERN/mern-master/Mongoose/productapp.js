//default basic mongoose config boilerplate
const mongoose = require('mongoose')
mongoose.connect('mongodb://localhost:27017/movieApp')
    .then (()=> {
        console.log('connection open')
    })
    .catch((err) => {
        console.log('error')
        console.log(err)
    })

//validating mongoose schema 
    const productSchema = new mongoose.Schema({
        name: {
            type: String, 
            required: true
        },
        //setting messages in arrays 
        price : {
            type: Number,
            required: true, 
            min: [0, 'price must be positive dumbass']
        },
        size: {
            type: String,
            enum: [S, M, L] //enum checks the value in array and throws a validationerror if it does not comply
        }
    });
    
//adding products in schema
    const Product = mongoose.model('Product', productSchema);
    const bike = new Product({ name: 'mtbike', price: 500})
    bike.save()
        .then(data => {
            console.log('it works')
            console.log(data);
        })
        .catch(err => {
            console.log('error')
            console.log(err)
        })

//updating mongoose schema
Product.findOneandUpdate({ name: 'tire pump'}, {price: -10}, {new: true})

//creating static model instance
productSchema.methods.greet = function() {
    console.log('hello there pussy')
}
const Product1 = mongoose.model('Product', productSchema)

const findProduct = async () => {
    const foundProduct = await Product.findOne({name: 'prc'});
    foundProduct.greet();
}

findProduct;
