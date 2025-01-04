const express = require('express');
const mongoose = require('mongoose');
const bodyParser = require('body-parser')
require('dotenv/config')

const app = express();
app.use(bodyParser.json());
//MidleWare
//app.use(AuthenticatorResponse)

const postRoute = require('./routes/posts')
app.use('/post', postRoute)

//You have the ability to create routes. 
app.get('/', (req, res) => {
    res.send('Hello World');
});


//connect to db 
mongoose.connect(process.env.DB_CONNECTION);

//How do we listen 

app.listen(5055);

