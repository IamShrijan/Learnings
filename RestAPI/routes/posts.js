const express = require('express');
const router = express.Router();
const post = require('../models/Post');
const Post = require('../models/Post');


router.get('/', async (req, res) => {
    try{
        const posts = await Post.find().limit(5);
        res.status(200).json(posts);
    }
    catch(err){
        res.status(500).json({message:err});
    }
});

router.post('/',async (req,res)=>{
    const post = new Post({
        title: req.body.title,
        description: req.body.description
    });
    try{
        const savedPost = await post.save();
        res.status(201).json(savedPost);
    }
    catch(err){
        res.status(500).json(err);
    }
    // post.save()
    // .then(data =>{
    //     res.status(201).json(data)
    // })
    // .catch(err =>{
    //     res.json({Message: err})
    // })

});

//getting Specific Post
router.get('/:postId', async(req,res)=>{
    try{
        const post = await Post.findById(req.params.postId);
        res.status(200).json(post);
    } catch(err){
        res.json({message:err});
    }
})
//delete Specific Post
router.delete('/:postId', async(req,res)=>{
    console.log(req.params.postId);
    try{
        const removedPost = await Post.findByIdAndDelete(req.params.postId);
        if (!removedPost) {
            return res.status(404).json({ message: "Post not found" });
        }
        res.status(200).json(removedPost);
    } catch(err){
        res.status(500).json({message:err});
    }
})

module.exports = router;