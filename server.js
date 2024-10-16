// server.js
// where your node app starts

// init project
var express = require('express');
const fs = require('fs');

var app = express();

require('dotenv').config();

// http://expressjs.com/en/starter/static-files.html
app.use("/pkg", express.static('pkg'));
app.use("/pointclouds", express.static('pointclouds'));

// http://expressjs.com/en/starter/basic-routing.html
app.get("/", function (request, response) {
  response.sendFile(__dirname + '/index.html');
});


// listen for requests :)
var listener = app.listen(process.env.PORT, function () {
  console.log(`Your app is listening on http://localhost:${listener.address().port}/`);
});
