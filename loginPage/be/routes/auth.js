const express = require('express');
const router  = express.Router();
const User = require('../models/User');


//Validation 
const Joi = require('@hapi/joi');
const schema= {
    name: Joi.string().min(6).required(),
    email: Joi.string().min(6).required().email(),
    password: Joi.string().min(6).required()
};


router.post('/register',async (req,res)=>{
    //Let us validate the data before making the user
    const validation = Joi.ValidationError(schema, req.body)
    if(validation){
        return res.send(validation);
    }
    const user = new User({
        name: req.body.name,
        email: req.body.email,
        password: req.body.password,
    });

    try{
        const savedUser = await user.save();
        res.status(200).send(savedUser);
    }catch(err){
        res.status(400).send(err)
    }
});

router.post('/login',(req,res)=>{
    res.send('login');
});


module.exports = router;