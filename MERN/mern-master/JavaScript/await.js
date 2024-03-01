//await keyword
//works inside async functions as well
async function rainbow() {
    await delayedColorChange('red', 1000)
    await delayedColorChange('orange', 1000)
    await delayedColorChange('green', 1000)
    await delayedColorChange('blue', 1000)
    await delayedColorChange('yellow', 1000)
    await delayedColorChange('indigo', 1000)
    return 'all done'
}
rainbow().then(() => console.log ('end of rainbow'));

async function printRainbow() {
    await rainbow();
    console.log('end of rainbow')
}