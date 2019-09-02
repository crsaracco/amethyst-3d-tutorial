# Amethyst 3D Tutorial

After having read [Amethyst](https://amethyst.rs/)'s official [mdbook tutorial](https://book.amethyst.rs/stable/), I've been excited to start diving in to Amethyst and trying some more stuff out, particularly when it comes to 3D game development.

Unfortunately, I left the official book feeling a little lost on how to do 3D stuff, and I've seen a few other people around the web express the same feelings. The [examples](https://github.com/amethyst/amethyst/tree/master/examples) have a lot of good content if you dig through them, but it's a little annoying having to piece together everything you need to do to get X done. So, I figured I'd create this tutorial! Worst-case, it'll serve as a little notebook for myself to reference, but hopefully it helps someone else out as well. :)

I'll be covering a lot of the same stuff that the official book covers (at least as far as basic Amethyst engine concepts are concerned), but I'm going to structure this tutorial a little differently than the book does. Instead of describing a lot of the basic concepts up front and then making a game with those concepts afterwards, I'm going to inline all of those explanations, discussing the concepts right before you use them.

If you want to, feel free to read [the Amethyst book](https://book.amethyst.rs/stable/) now (especially everything up to and including chapter 3: "Concepts") to get a feel for the Amethyst ecosystem. My goal is to eventually cover everything that the book does, though, so if you want to skip it and dive right in to 3D game development, I hope this tutorial explains everything well enough.

Also, as a side-note, I'm going to postpone the use of assets and config files until much later in this tutorial compared to the book, because I think it makes it easier to understand what's going on in the Amethyst ecosystem that way. I will be covering that stuff later in the tutorial, so don't fear!

## Disclaimer

I'm still very much a noob at Amethyst, so if you notice something that I misunderstood, feel free to send me an issue or pull request on [this tutorial's GitHub repo](https://github.com/crsaracco/amethyst-3d-tutorial). Same thing for typos and other errors.

This tutorial assumes you're already familiar with Rust development. If you aren't, I'd recommend reading a little bit of [the Rust book](https://doc.rust-lang.org/book/) and playing around with Rust a little.

## What game are we implementing here?

*"Back in the day"™*, I played a decent amount of flash games. But now, flash is dead. RIP.

One of those games was **Cubefield** ([website](http://www.cubefield.org.uk/)) ([youtube video example](https://www.youtube.com/watch?v=s80HExhyIng)), a very simple obstacle-avoiding game (in the genre of "endless running" games), similar to [Race The Sun](https://www.youtube.com/watch?v=eDLfSg3YwVQ0) and others.

I figure Cubefield would be a pretty simple target for a 3D tutorial: the objective of the game is pretty simple, the shapes are all pretty simple, and even the controls are simple. Even so, we should be able to cover a lot of the concepts that you need to go off and create your own 3D game.

## Additional resources

Here's some other resources that you might find useful in developing games with Amethyst:

 - [Website](https://amethyst.rs/)
    - the ["Documentation" page](https://amethyst.rs/doc) lists a lot of the same stuff that I have here
 - [the Amethyst book](https://book.amethyst.rs/stable/)
 - [API reference](https://docs.amethyst.rs/stable/amethyst/)
 - [GitHub repo](https://github.com/amethyst/amethyst)
    - especially [the `examples` directory](https://github.com/amethyst/amethyst/tree/v0.12.0/examples)
 - [Discord chat](https://discordapp.com/invite/amethyst): A lot of game devs (including a lot of the people who work on Amethyst) hang out on Discord, so it might be a good place to ask questions.
 - [Discourse forums](https://community.amethyst.rs/): Alternatively, you could also ask questions in the Discourse forums.
