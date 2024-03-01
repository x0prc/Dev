const mongoose = require('mongoose')
mongoose.connect('mongodb://localhost:27017/relationshipDemo')
    .then(() => {
        console.log('mongo connected')
    })
    .catch(err => {
        console.log('mongo connection error')
        console.log(err)
    })

const productSchema = new mongoose.Schema({
    name: String, 
    price: Number, 
    season: {
        type: String, 
        enum: ['Spring', 'Summer', 'Fall', 'Winter']
    }
});

const Product = mongoose.model('Product', productSchema); 
Product.insertMany([
    {name: 'Melon', price: 4, season: 'Summer'},
    {name: 'sugar', price: 3, season: 'Spring'},
])

const farmSchema = new mongoose.Schema({
    name: String, 
    city: String,
    products: [{ type: Schema.Types.ObjectId, ref: 'Product' }]
})


//populating produts with mongoose (one to many)
const addProduct = async() => {
    const farm = await farm.findOne({name: 'Farmz'});
    const watermelon = await Product.findOne({name: 'sugar watermelon'})
    farm.products.push(watermelon);
    await farm.save();
    console.log(farm);
}

farm.findOne({name: 'farmasz'})
.populate('products')
.then(farm => console.log(farm))