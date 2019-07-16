'use strict';

require("./styles.scss");

const { Elm } = require('./Main');

var app = Elm.Main.init({
    flags: `\
Title:
    Alien
Author:
    Dan O'Bannon
INT. MESS

The entire crew is seated. Hungrily swallowing huge portions of artificial food. The cat eats from a dish on the table.

KANE
First thing I'm going to do when we get back is eat some decent food.
`});

app.ports.toJs.subscribe(data => {
    console.log(data);
})
// Use ES2015 syntax and let Babel compile it for you
var testFn = (inp) => {
    let a = inp + 1;
    return a;
}
