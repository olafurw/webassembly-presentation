@standard

# **Test slide. Can you read this?**

Sample text size!
- Sample bullet text size.

.    
Sample code color and size.
![](/images/sample-code.png)

text will go about this far down

@@@
@title

# **Sandbox Games**

Using WebAssembly and C++ to make a simple game

@@@
@promo

![](/images/me.png)

# Ólafur Waage

Senior Software Developer - TurtleSec AS    
<span style="color:#4DBFBA;font-weight:bold;">@olafurw</span> on Twitter    
<span style="color:#4DBFBA;font-weight:bold;">@olafurw@mastodon.social</span> on Mastodon

@@@
@image-fullscreen

![](/images/turtlesec.png)

@@@
@quote

*"I must go forward where I have never been*    
*instead of backwards where I have."*    

@@@
@quote

*"I must go forward where I have never been*    
*instead of backwards where I have."*    
**Winnie the Pooh**

@@@
@standard

# WHAT THIS TALK IS AND ISN’T

This is not a game development or game design talk. A while ago I was making a game using WebAssembly and these are the walls I encountered along the way

@@@
@standard

# WHAT THIS TALK IS AND ISN’T

This is not a game development or game design talk. A while ago I was making a game using WebAssembly and these are the walls I encountered along the way

This is not a comprehensive talk about WebAssembly. 

@@@
@standard

# WHAT THIS TALK IS AND ISN’T

This is not a game development or game design talk. A while ago I was making a game using WebAssembly and these are the walls I encountered along the way

This is not a comprehensive talk about WebAssembly. 

The idea here is to be pragmatic and learn what this tool has to offer and what problems it can solve.

@@@
@title

# What is WebAssembly?

@@@
@title

# What is WebAssembly?

How can something be neither Web nor Assembly?

@@@
@standard

# WHAT IS WEBASSEMBLY?

WebAssembly is a binary format* originally designed to allow for performant execution of code within browsers.

@@@
@standard

# WHAT IS WEBASSEMBLY?

WebAssembly is a binary format* originally designed to allow for performant execution of code within browsers.

- Announced 2015

@@@
@standard

# WHAT IS WEBASSEMBLY?

WebAssembly is a binary format* originally designed to allow for performant execution of code within browsers.

- Announced 2015
- Working Drafts in 2018

@@@
@standard

# WHAT IS WEBASSEMBLY?

WebAssembly is a binary format* originally designed to allow for performant execution of code within browsers.

- Announced 2015
- Working Drafts in 2018
- W3C recommendation in 2019

@@@
@standard

# WHAT IS WEBASSEMBLY?

WebAssembly is a binary format* originally designed to allow for performant execution of code within browsers.

- Announced 2015
- Working Drafts in 2018
- W3C recommendation in 2019

WebAssembly can be thought of as the target output of any language and in recent times can be executed outside of the web.

@@@
@image-fullscreen

![](/images/wasm-code-1.png)

@@@
@image-fullscreen

![](/images/wasm-code-2.png)

@@@
@standard

# WEBASSEMBLY EXAMPLES?

Many of you might associate WebAssembly with games only, and even though this talk is also doing that, WebAssembly has so much more to offer.

@@@
@standard

# WEBASSEMBLY EXAMPLES?

Many of you might associate WebAssembly with games only, and even though this talk is also doing that, WebAssembly has so much more to offer.

Here are some examples of things you might not have thought are written with WebAssembly.

@@@
@image-fullscreen

![](/images/example-1.png)

@@@
@image-fullscreen

![](/images/example-2.png)

@@@
@image-fullscreen

![](/images/example-3.png)

@@@
@image-fullscreen

![](/images/example-4.png)

@@@
@image-fullscreen

![](/images/example-5.png)

@@@
@title

# What is Emscripten?

@@@
@title

# What is Emscripten?

WebAssembly before WebAssembly

@@@
@standard

# WHAT IS EMSCRIPTEN?

We originally had asm.js from Mozilla which had similar goals to WebAssembly, to run efficient code on the web.

@@@
@standard

# WHAT IS EMSCRIPTEN?

We originally had asm.js from Mozilla which had similar goals to WebAssembly, to run efficient code on the web.

asm.js is a subset of JavaScript and your lower level code would then be transpiled into it.

@@@
@standard

# WHAT IS EMSCRIPTEN?

We originally had asm.js from Mozilla which had similar goals to WebAssembly, to run efficient code on the web.

asm.js is a subset of JavaScript and your lower level code would then be transpiled into it.

This is where Emscripten came into play.

@@@
@standard

# WHAT IS EMSCRIPTEN?

Emscripten is based on the LLVM/Clang toolchains which allows you target WebAssembly as the binary output.

@@@
@standard

# WHAT IS EMSCRIPTEN?

Emscripten is based on the LLVM/Clang toolchains which allows you target WebAssembly as the binary output.

This allows you to get many different types of outputs, not only WASM files but .js and .html

@@@
@standard

# INSTALLING EMSCRIPTEN

Let’s go over the installation process and setup a simple development environment.

@@@
@standard

# INSTALLING EMSCRIPTEN

Let’s go over the installation process and setup a simple development environment.

- Text editor is VSCode

@@@
@standard

# INSTALLING EMSCRIPTEN

Let’s go over the installation process and setup a simple development environment.

- Text editor is VSCode
- WSL2 running Ubuntu 20.04

@@@
@standard

# INSTALLING EMSCRIPTEN

Let’s go over the installation process and setup a simple development environment.

- Text editor is VSCode
- WSL2 running Ubuntu 20.04
- https://github.com/olafurw/talk-accu-webassembly


@@@
@image-fullscreen

![](/images/install-1.png)


@@@
@image-fullscreen

![](/images/install-2.png)


@@@
@image-fullscreen

![](/images/install-3.png)


@@@
@image-fullscreen

![](/images/install-4.png)


@@@
@image-fullscreen

![](/images/install-5.png)


@@@
@image-fullscreen

![](/images/install-6.png)

@@@
@image-fullscreen

![](/images/install-7.png)

@@@
@image-fullscreen

![](/images/installed-1.png)

@@@
@image-fullscreen

![](/images/installed-2.png)

@@@
@image-fullscreen

![](/images/installed-3.png)

@@@
@image-fullscreen

![](/images/installed-4.png)

@@@
@standard

# HEY, WORLD, WHAT IS UP?

Now we have the Emscripten compiler installed in our system.

@@@
@standard

# HEY, WORLD, WHAT IS UP?

Now we have the Emscripten compiler installed in our system.

Time for the time honored tradition of the hello world example.

@@@
@standard

# HEY, WORLD, WHAT IS UP?

Now we have the Emscripten compiler installed in our system.

Time for the time honored tradition of the hello world example.

But there are a few more steps in this one than you’d normally expect.

@@@
@image-fullscreen-dark

![](/images/helloworld-1.png)

@@@
@image-fullscreen-dark

![](/images/helloworld-2.png)

@@@
@image-fullscreen-dark

![](/images/helloworld-3.png)

@@@
@image-fullscreen-dark

![](/images/helloworld-more-1.png)

@@@
@image-fullscreen-dark

![](/images/helloworld-more-2.png)

@@@
@image-fullscreen-dark

![](/images/helloworld-more-3.png)

@@@
@image-fullscreen-dark

![](/images/helloworld-more-4.png)

@@@
@standard

# JUST RUN IT ALREADY!

Yes, with nodejs we can run the .js files just fine.

@@@
@standard

# JUST RUN IT ALREADY!

Yes, with nodejs we can run the .js files just fine.

But let’s start by opening the HTML file directly. Should be no problem, right?

@@@
@title

# WALL NUMBER 1

@@@
@title

# WALL NUMBER 1

Of CORS there’s a problem here

@@@
@standard

# YOUR SAFETY IS PARAMOUNT

Browsers don’t like opening random files from whatever location you decide.

@@@
@standard

# YOUR SAFETY IS PARAMOUNT

Browsers don’t like opening random files from whatever location you decide.

There’s a thing called "Cross-origin resource sharing (CORS)". By default browsers don’t like loading external files from disk using file://

@@@
@standard

# YOUR SAFETY IS PARAMOUNT

Browsers don’t like opening random files from whatever location you decide.

There’s a thing called "Cross-origin resource sharing (CORS)". By default browsers don’t like loading external files from disk using file://

The browser will load the html file fine but any external dependency will probably be blocked.

@@@
@standard

# RUN EM RUN!

Best way to solve this is to run a webserver that is going to host the files.

@@@
@standard

# RUN EM RUN!

Best way to solve this is to run a webserver that is going to host the files.

What I use while developing is emrun, a tool that comes with emscripten.

@@@
@standard

# RUN EM RUN!

Best way to solve this is to run a webserver that is going to host the files.

What I use while developing is emrun, a tool that comes with emscripten.

emrun is a simple webserver but for our development purposes it is good enough.

@@@
@image-fullscreen-dark

![](/images/helloworld-run-1.png)

@@@
@image-fullscreen-dark

<iframe class="example-frame-1" src="http://127.0.0.1:8001/hello_world.html" height="800" width="800"></iframe>

![](/images/helloworld-run-2.png)

@@@
@standard

# VIDEO GAMES!

Now let’s look at the game we will be “making”.

@@@
@standard

# VIDEO GAMES!

Now let’s look at the game we will be “making”.

We are going to make a simple sliding puzzle game, similar to games like “Threes” and “2048”

@@@
@standard

# VIDEO GAMES!

Now let’s look at the game we will be “making”.

We are going to make a simple sliding puzzle game, similar to games like “Threes” and “2048”

<iframe class="example-frame-2" scrolling="no" src="http://localhost:8002/index.html" height="420" width="420"></iframe>

@@@
@standard

# ENOUGH FUN

Now let’s covert this game over to WebAssembly.

@@@
@standard

# ENOUGH FUN

Now let’s covert this game over to WebAssembly.

There are two ways to do this:

@@@
@standard

# ENOUGH FUN

Now let’s covert this game over to WebAssembly.

There are two ways to do this:
- Keep the drawing in JS and game logic in C++

@@@
@standard

# ENOUGH FUN

Now let’s covert this game over to WebAssembly.

There are two ways to do this:
- Keep the drawing in JS and game logic in C++
- Do everything in C++

@@@
@standard

# ENOUGH FUN

Now let’s covert this game over to WebAssembly.

There are two ways to do this:
- Keep the drawing in JS and game logic in C++
- Do everything in C++

We will look at both, and the walls we hit along the way.

@@@
@standard

# LET’S START CONVERTING

So let’s take some of the functions we have in the JS version and convert them over to C++

@@@
@standard

# LET’S START CONVERTING

So let’s take some of the functions we have in the JS version and convert them over to C++

Some of them don’t even need to know about game state, so let’s start with them.

@@@
@image-fullscreen-dark

![](/images/game-code-1.png)

@@@
@image-fullscreen-dark

![](/images/game-code-2-1.png)

@@@
@image-fullscreen-dark

![](/images/game-code-2-2.png)

@@@
@image-fullscreen-dark

![](/images/game-code-2.png)

@@@
@image-fullscreen-dark

![](/images/game-code-build.png)

@@@
@image-fullscreen-dark

![](/images/game-code-3-1.png)

@@@
@image-fullscreen-dark

![](/images/game-code-3-2.png)

@@@
@image-fullscreen-dark

![](/images/game-code-3-3.png)

@@@
@image-fullscreen-dark

![](/images/game-code-1.png)

@@@
@title

# WALL NUMBER 2

@@@
@title

# WALL NUMBER 2

Where we’re going, there is no OS

@@@
@standard

# SO RANDOM

With this standalone WASM file, there is no operating system level functionality.

@@@
@standard

# SO RANDOM

With this standalone WASM file, there is no operating system level functionality.

You’re all on your own*

@@@
@standard

# SO RANDOM

With this standalone WASM file, there is no operating system level functionality.

You’re all on your own*

So how do we solve this problem?

@@@
@standard

# EMSCRIPTEN SAVIORS

Using random, calling timer functions and many other OS level functionality has to come from somewhere.

@@@
@standard

# EMSCRIPTEN SAVIORS

Using random, calling timer functions and many other OS level functionality has to come from somewhere.

Thankfully there is a solution to this, where if you build a .js file in addition to your .wasm file, you will get many of these functionalities from the javascript side.

@@@
@standard

# EMSCRIPTEN SAVIORS

Using random, calling timer functions and many other OS level functionality has to come from somewhere.

Thankfully there is a solution to this, where if you build a .js file in addition to your .wasm file, you will get many of these functionalities from the javascript side.

But how does it work? Can we do it ourselves?

@@@
@standard

# EMSCRIPTEN RANDOM

![](/images/random.png)

Looks great, but how do we use it?

@@@
@image-fullscreen-dark

![](/images/game-code-4-1.png)

@@@
@image-fullscreen-dark

![](/images/game-code-4-2.png)

@@@
@image-fullscreen-dark

![](/images/game-code-4-3.png)

@@@
@image-fullscreen-dark

![](/images/game-code-5-1.png)

@@@
@image-fullscreen-dark

![](/images/game-code-5-2.png)

@@@
@image-fullscreen-dark

![](/images/game-code-6-1.png)

@@@
@image-fullscreen-dark

![](/images/game-code-6-2.png)

@@@
@image-fullscreen-dark

![](/images/game-code-6-3.png)

@@@
@image-fullscreen-dark

![](/images/game-code-6-4.png)

@@@
@image-fullscreen-dark

![](/images/game-code-6-5.png)

@@@
@image-fullscreen-dark

![](/images/game-code-7-1.png)

@@@
@standard

# ONWARDS

Great, so now we can move over the rest of the game logic.

@@@
@standard

# ONWARDS

Great, so now we can move over the rest of the game logic.

The board is an array of arrays of `Box` and the rest of the game logic is basically identical.

@@@
@standard

# ONWARDS

Great, so now we can move over the rest of the game logic.

The board is an array of arrays of `Box` and the rest of the game logic is basically identical.

So now the gameplay can be simulated and called from JS, now we need to draw that data.

@@@
@title

# WALL NUMBER 3

@@@
@title

# WALL NUMBER 3

Where’s the data?

@@@
@standard

# I REMEMBER

We can communicate between C++ and JS using primitive types as you saw before, but as soon as things get a bit more complicated, we are in trouble.

@@@
@standard

# I REMEMBER

We can communicate between C++ and JS using primitive types as you saw before, but as soon as things get a bit more complicated, we are in trouble.

We could view the raw data of a std::vector within the memory of WebAssembly, but converting between a vector and a javascript list is not automatic

@@@
@standard

# WE’RE IN A BIND

There is something called Embind that can help with passing more complex objects over to JS

![](/images/bind-class.png)

@@@
@standard

# WE’RE IN A BIND

Embind even has helpers to bind common objects, like std::vector

![](/images/bind-vector.png)

@@@
@standard

# I REMEMBER

You can even define a shared block of memory that can then be used by either JS or C++

@@@
@standard

# I REMEMBER

You can even define a shared block of memory that can then be used by either JS or C++

Also there is the option to return a pointer to JS

@@@
@standard

# I REMEMBER

You can even define a shared block of memory that can then be used by either JS or C++

Also there is the option to return a pointer to JS

But this is in the territory where you need to be a bit more careful with how each byte is used and represented.

@@@
@image-fullscreen

![](/images/webassembly-portability.png)

@@@
@standard

# WE DON’T NEED IT

Thankfully, I wrote the game logic to only use simple primitives, so we can finish converting all of the functions over to C++ and expose them to JS to use as needed.

@@@
@standard

# WE DON’T NEED IT

Thankfully, I wrote the game logic to only use simple primitives, so we can finish converting all of the functions over to C++ and expose them to JS to use as needed.

Let’s look at this version of the implementation.

@@@
@image-fullscreen

<iframe class="example-frame-1" src="http://127.0.0.1:8003/index.html" height="420" width="420"></iframe>

@@@
@standard

# LET’S NOT STOP HERE!

Now we have basically everything except the rendering in the C++ version.

@@@
@standard

# LET’S NOT STOP HERE!

Now we have basically everything except the rendering in the C++ version.

So let’s move that over as well.

@@@
@standard

# LET’S NOT STOP HERE!

Now we have basically everything except the rendering in the C++ version.

So let’s move that over as well.

Thankfully Emscripten has great support for exactly what we need.

@@@
@standard

# SDL1 and 2

Emscripten has built in support for SDL which is a cross platform library that provides among many things graphical rendering support.

@@@
@standard

# SDL1 and 2

Emscripten has built in support for SDL which is a cross platform library that provides among many things graphical rendering support.

There is also support for SDL2 but it needs to be downloaded (which happens on first compile)

@@@
@standard

# SDL1 and 2

Emscripten has built in support for SDL which is a cross platform library that provides among many things graphical rendering support.

There is also support for SDL2 but it needs to be downloaded (which happens on first compile)

![](/images/sdl1-2.png)

@@@
@standard

# GLUE THAT CODE

Also since we will use SDL2 and other built in functionality, we will use the generated JS glue code.

@@@
@standard

# GLUE THAT CODE

Also since we will use SDL2 and other built in functionality, we will use the generated JS glue code.

So instead of creating the importObject ourselves and implementing the functions that are needed, Emscripten has does this for us.

@@@
@image-fullscreen-dark

![](/images/game-code-main.png)

@@@
@standard

# RENDERING FUN

Now I port over the rendering code, which thankfully for this example is just a simple colored rectangle. (I wait with displaying the text for now)

@@@
@image-fullscreen-dark

![](/images/sdl-draw-1.png)

@@@
@image-fullscreen-dark

![](/images/sdl-draw-2.png)

@@@
@standard

# RENDERING FUN

Everything compiles and looks like it should be.

@@@
@standard

# RENDERING FUN

Everything compiles and looks like it should be.

I run the code, I see the box

@@@
@standard

# RENDERING FUN

Everything compiles and looks like it should be.

I run the code, I see the box

![](/images/before-text.png)

@@@
@standard

# RENDERING FUN

Everything compiles and looks like it should be.

I run the code, I see the box

![](/images/before-text.png)

and then...

@@@
@image-fullscreen

![](/images/game-code-memory-error.png)

@@@
@title

# WALL NUMBER 4

@@@
@title

# WALL NUMBER 4

The sandbox isn't infinite

@@@
@standard

# MEMORY MANAGEMENT

Up to this point I have been using the default memory size and it has just happened to fit.

@@@
@standard

# MEMORY MANAGEMENT

Up to this point I have been using the default memory size and it has just happened to fit.

But we need more memory now since SDL is involved.

@@@
@standard

# MEMORY MANAGEMENT

Up to this point I have been using the default memory size and it has just happened to fit.

But we need more memory now since SDL is involved.

![](/images/memory-manage.png)

@@@
@standard

# TEXT ADVENTURE

Great, this compiles and we see the box drawn in the canvas as before.

@@@
@standard

# TEXT ADVENTURE

Great, this compiles and we see the box drawn in the canvas as before.

So let’s draw the text that should appear within the box.

@@@
@title

# WALL NUMBER 5

@@@
@title

# WALL NUMBER 5

File not found

@@@
@standard

# EMPTY SANDBOX

The environment we are in does not have much else outside of what we have given it.

@@@
@standard

# EMPTY SANDBOX

The environment we are in does not have much else outside of what we have given it.

So the font file we want to use does not exist, and the idea of a filesystem is different from what we expect. We have to provide the files.

@@@
@standard

# EMPTY SANDBOX

The environment we are in does not have much else outside of what we have given it.

So the font file we want to use does not exist, and the idea of a filesystem is different from what we expect. We have to provide the files.

![](/images/preload-file.png)

@@@
@standard

# EMPTY SANDBOX

The environment we are in does not have much else outside of what we have given it.

So the font file we want to use does not exist, and the idea of a filesystem is different from what we expect. We have to provide the files.

![](/images/preload-file-2.png)

@@@
@standard

# CMAKE

What Emscripten also provides is helper utilities to use common development tools like make and cmake.

@@@
@standard

# CMAKE

What Emscripten also provides is helper utilities to use common development tools like make and cmake.

So I also wrote a simple CMake file for building the project.

@@@
@standard

# CMAKE

What Emscripten also provides is helper utilities to use common development tools like make and cmake.

So I also wrote a simple CMake file for building the project.

![](/images/emcmake.png)

@@@
@image-fullscreen-dark

![](/images/cmake.png)

@@@
@standard

# IT’S RUNNING!

Great! So now we have everything running.

Let’s look at it in action!

@@@
@standard

# IT’S RUNNING!

Great! So now we have everything running.

Let’s look at it in action!

<iframe class="example-frame-2" scrolling="no" src="http://localhost:8004/build/game.html" height="420" width="450"></iframe>

@@@
@standard

# SUMMARY

Let’s summarize the walls we encountered:

@@@
@standard

# SUMMARY

Let’s summarize the walls we encountered:

- Files need to be served while developing

@@@
@standard

# SUMMARY

Let’s summarize the walls we encountered:

- Files need to be served while developing
- All functionality you depend on (ie. OS) needs to be implemented or given to you

@@@
@standard

# SUMMARY

Let’s summarize the walls we encountered:

- Files need to be served while developing
- All functionality you depend on (ie. OS) needs to be implemented or given to you
- Data needs to be primitives or converted in some way before sending to JS

@@@
@standard

# SUMMARY

Let’s summarize the walls we encountered:

- Files need to be served while developing
- All functionality you depend on (ie. OS) needs to be implemented or given to you
- Data needs to be primitives or converted in some way before sending to JS
- Memory size and growth needs to be thought about

@@@
@standard

# SUMMARY

Let’s summarize the walls we encountered:

- Files need to be served while developing
- All functionality you depend on (ie. OS) needs to be implemented or given to you
- Data needs to be primitives or converted in some way before sending to JS
- Memory size and growth needs to be thought about
- Required files need to be embedded or preloaded with the output

@@@
@image-fullscreen

![](/images/columbo.png)

@@@
@promo

![](/images/me.png)

# Ólafur Waage

Senior Software Developer - TurtleSec AS    
<span style="color:#4DBFBA;font-weight:bold;">@olafurw</span> on Twitter    
<span style="color:#4DBFBA;font-weight:bold;">@olafurw@mastodon.social</span> on Mastodon    
https://github.com/olafurw/talk-accu-webassembly
https://github.com/olafurw/webassembly-presentation
