const express = require('express');
const router = express.Router();

router.get('/', (req, res) => {
    res.send('all dogs')
})

router.get('/:id', (req, res) => {
    res.send('one dog')
})

router.get('/:id/edit', (req, res) => {
    res.send('editing one dog')
})

module.exports = router;
