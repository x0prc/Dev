const mongoose = require('mongoose')
mongoose.connect('mongodb://localhost:27017/movieApp')
    .then (()=> {
        console.log('connection open')
    })
    .catch((err) => {
        console.log('error')
        console.log(err)
    })

//virtuals in databases in mongoose
const personSchema = new mongoose.Schema({
    first: String,
    last: String
})

personSchema.virtual('fullName').get(function () {
    return `${this.first} ${this.last}`
})

//pre and post save (to run code before and after something happens)
personSchema.pre('save', async function() {
    this.first = 'yo';
    this.last = 'mama';
    console.log('about to save')
})

personSchema.post('save',async function() {
    console.log('saved')
})

const Person = mongoose.model('Person', personSchema);
