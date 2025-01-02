const express = require('express');

const app = express();

//You have the ability to create routes. 
app.get('/', (req, res) => {
    res.send('Hello World');
});
app.post('/', (req, res) => {
    res.send('Hello World');
});
//How do we listen 
app.listen(5055);

