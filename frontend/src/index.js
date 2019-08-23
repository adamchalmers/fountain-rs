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

PARKER
I've had worse than this, but I've had better too, if you know what I mean.

LAMBERT
Christ, you're pounding down this stuff like there's no tomorrow.

Pause.

PARKER
I mean I like it.

KANE
No kidding.

PARKER
Yeah.  It grows on you.

KANE
It should.  You know what they make this stuff out of...

PARKER
I know what they make it out of. So what. It's food now. You're eating it.

Suddenly Kane grimaces.

RIPLEY
What's wrong.

Kane's voice strains.

LAMBERT
What's the matter.

KANE
I don't know... I'm getting cramps.

The others stare at him in alarm. Suddenly he makes a loud groaning noise. Clutches the edge of the table with his hands. Knuckles whitening.

ASH
Breathe deeply.

Kane screams.

KANE
Oh God, it hurts so bad. It hurts.  It hurts.

KANE
(stands up)
Ooooooh.

BRETT
What is it.  What hurts.

Kane's face screws into a mask of agony. He falls back into his chair.

KANE
Ohmygooaaaahh.
`});

app.ports.toJs.subscribe(data => {
    console.log(data);
})
// Use ES2015 syntax and let Babel compile it for you
var testFn = (inp) => {
    let a = inp + 1;
    return a;
}
