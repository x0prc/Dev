//better than factory approach.
const navColor = new Color('carrot', [230, 126, 34]);
const logoColor = new Color('emerald', [34, 203, 114]);


//converts to object.
function Color(r,g,b) {
    this.r = r; 
    this.g = g; 
    this.b = b; 
    console.log(this);
}

//creating rgb prototypes.
Color.prototype.rgb = function() {
    const {r, g, b} = this;
    return '#' + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1);
}