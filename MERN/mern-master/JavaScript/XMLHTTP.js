const req = new XMLHttpRequest();
req.onload = function() {
    console.log('it loads');
    const data = JSON.parse(this.responseText)
    console.log(data.name, data.height);
};

req.onerror = function () {
    console.log('error');
    console.log(this);
};

req.open('get', 'https://swapi.dev/api/people/1');
req.send();

