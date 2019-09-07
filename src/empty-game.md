# Creating an empty game

In this chapter, I'm going to go from "having no project at all" to "having a game that does nothing except spawn a blank Amethyst window". Getting this basic game skeleton stuff out of the way will allow us to focus on concepts and game functionality in future chapters.

## Create a Rust project

First, you need to create a Rust project.

```
cargo new cubefield
```

Phew, that was a lot of work.

## Set up Amethyst dependencies and the `main` function

First thing's first, you need to add `amethyst` to your project's dependencies. You also need to choose a backend:

 - `vulkan` for Linux and Windows (and anything else that supports Vulkan)
 - `metal` for Mac OS X
 - `dx12` (for DirectX) is ***not*** currently available in Amethyst, but might be one day.

Since I'm on Linux, I'll be using `vulkan`.

**Cargo.toml:**

```toml
[package]
# ...

[dependencies]
amethyst = {version="0.12.0", features["vulkan"]}
```

To make a few things easier, we're also going to change the function signature of `main`. Most Amethyst games use the return type `amethyst::Result<()>`, so that you can use `?` to automatically exit if various setup things fail.

**main.rs:**

```rust
fn main() -> amethyst::Result<()> {
    Ok(())
}
```

Go ahead and `cargo run` that, and wait for Cargo to pull down the giant dependency tree and compile it. Depending on your computer's specs, it might take *a while*.

If everything worked, congrats! You now have some code that depends on Amethyst. But it doesn't do anything yet...

## Create a logger

Setting up a logger allows us to see what's going on in our game while it's running. It's particularly useful while you're still laying the groundwork of your game, as we are here: any debug, info, and warning message from within the Amethyst ecosystem will be printed out to the screen.

```rust
fn main() -> amethyst::Result<()> {
    // Set up the Amethyst logger
    amethyst::start_logger(Default::default());

    Ok(())
}
```

As the Amethyst book says, there are many ways to configure the logger. [Check out the API reference](https://docs-src.amethyst.rs/stable/amethyst/struct.Logger.html) for more info. The default setup will work just fine for this tutorial, though.

## Create an `Application` object

The [API reference](https://docs-src.amethyst.rs/stable/amethyst/type.Application.html) describes the `Application` object pretty well:

> An Application is the root object of the game engine. It binds the OS event loop, state machines, timers and other core components in a central place.
>
> Since Application functions as the root of the game, Amethyst does not need to use any global variables. Within this object is everything that your game needs to run.

Every Amethyst game creates an `Application`, so that seems like a good place to start.

In order to create an `Application`, we first have to create a few bits of information that it depends on:

 - A `PathBuf` to an assets directory
 - An initial `State` to start the game in
 - A `GameDataBuilder` object; `GameData` holds a bunch of game logic and other stuff that your game needs during runtime.

#### Assets directory

First, we need to create a `PathBuf` that represents an assets directory. Assets are anything that your game loads before/during game runtime -- things like 3D models, textures, audio, sprites, etc. Pretty much any game of any substantial size will have an assets directory... but we won't need to have one for quite a while in this tutorial! Let's just give it a sane directory name (`assets`) for now, and I'll describe how assets work when we start using them.

```rust
use amethyst::utils::application_root_dir;

fn main() -> amethyst::Result<()> {
    // (logger)

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    Ok(())
}
```

By the way, you don't actually *need* to create the `assets/` directory now (in the root directory of your project, at the same level as `src/`), but you can if you want.

#### Create an initial `State`

Amethyst has a neat style of state management built-in called a "pushdown automaton", which you can read a little more about on [Wikipedia](https://en.wikipedia.org/wiki/Pushdown_automaton) and [Amethyst's wiki page](https://github.com/amethyst/amethyst/wiki/Game-State-Machine-Design). It's a little different than the (maybe?) more-common "[finite-state machine](https://en.wikipedia.org/wiki/Finite-state_machine)", but pushdown automatons are a little more powerful.

Regardless, we won't be needing the features of the state management system quite yet. Simple games like this can have just one main "game" state; once you want to start implementing main menus, pause menus, etc, *then* you'll probably want to start reaching for state management. Let's just satisfy `Application` for now.

The easiest way to create a `State` in Amethyst is to define a unit struct and implement the `SimpleState` trait for it:

```rust
// (other includes)
use amethyst::SimpleState;

struct GameState;
impl SimpleState for GameState {}

fn main() -> amethyst::Result<()> {
    // (...)
}
```

#### Create a `GameDataBuilder`

`GameData` is the "central repository" for a bunch of stuff your game needs during runtime, and `GameDataBuilder` is a convenient way to build it. For now, we don't need anything in the `GameData`, so we can just create a builder with nothing in it. We'll be adding stuff to it very soon (before the end of this chapter, even).

```rust
// (other includes)
use amethyst::GameDataBuilder;

// (GameState)

fn main() -> amethyst::Result<()> {
    // (logger)
    // (assets directory)

    // Set up an empty (for now) GameDataBuilder
    let game_data = GameDataBuilder::default();

    Ok(())
}
```

#### Create an `Application`, and run our game!

We're finally ready to create an `Application`!

```rust
// (other includes)
use amethyst::Application;

// (GameState)

fn main() -> amethyst::Result<()> {
    // (logger)
    // (assets directory)
    // (GameDataBuilder)

    // Run the game!
    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
```

Go ahead and `cargo run` this to make sure everything's working. You should just see a few log messages print out before Amethyst dumps you into an infinite loop. `Ctrl+C`.

## Spawn a window

Alright, now we need a window for our game.

To do this, we're going to bring in a "[Bundle](https://docs.amethyst.rs/stable/amethyst_core/bundle/trait.SystemBundle.html)" that implements a lot of the stuff for us. In general, you can think of bundles as a cross-section of game functionality that implements some feature for you. It's possible to (and you will likely want to eventually) create your own bundles, but that's a bit far away for now.

In particular, we're going to be using the `RenderingBundle` here, which (along with some bundle plugins) implements a bunch of rendering stuff for us, including window creation itself.

```rust
// (other includes)
use amethyst::renderer::plugins::RenderPbr3D;
use amethyst::renderer::plugins::RenderToWindow;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::window::DisplayConfig;

// (GameState)

fn main() -> amethyst::Result<()> {
    // (logger)
    // (assets directory)

    // Set up the display configuration
    let display_config = DisplayConfig {
        title: "Amethyst".to_string(),
        dimensions: Some((1024, 768)),
        ..Default::default()
    };

    // Set up the GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.529, 0.808, 0.98, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?;

    // (Application)
    Ok(())
}
```

That's a lot of new code, so let's break it down line-by-line.

 - The [DisplayConfig](https://docs.amethyst.rs/stable/amethyst_window/struct.DisplayConfig.html) is what will eventually get used by the `RenderToWindow` plugin to set up our window for us. Here we're setting the window's title and dimensions explicitly, then using the defaults for every other field in the struct.
 - We already had `GameDataBuilder` before, but now we're adding a bundle to it. `with_bundle` is the function you use to add a bundle.
 - `RenderingBundle` is the bundle we're using to set up rendering. As far as bundles go, `RenderingBundle` is a pretty complex one: it delegates most of its work to plugins, which lets the user set up their rendering system as they see fit. It's also parameterized on a "Backend", but since you set up your rendering backend in `Cargo.toml`, you can just use `DefaultBackend` here.
 - Since `RenderingBundle` delegates most of its work to plugins, we need to give it some plugins. `with_plugin` is the function you use to add plugins, at least for `RenderingBundle`.
 - The first plugin we're using is `RenderToWindow`, which sets up a window for us and allows us to draw to it. You have to supply a `DisplayConfig` for it to use, which is what we did up above. We're also specifying a "clear value", which is what the renderer uses as a "background" to your game when there was nothing in front of it to render.
    - The format for the color is `[Red, Green, Blue, Alpha]`. You'll probably want to set alpha to `1.0`, and the rest of the colors are up to you. Here I made light sky-blue color.
    - If you don't specify a clear value, the renderer won't clear the screen, just drawing over the last frame directly. It'll look similar to [the classic Windows lagging graphics glitch](https://www.youtube.com/watch?v=ZjlSJQBdojo). Try commenting that line of code out when you have more of a game to work with!
 - The second plugin is `RenderPbr3D`, which is a [Physically Based Rendering](https://en.wikipedia.org/wiki/Physically_based_rendering) engine that we'll be using for rendering our game.

Go ahead and `cargo run` that. You should get a blank window, filled with whatever your clear color is.

## Final code for this chapter

The final code for this chapter can be found [here](https://github.com/crsaracco/amethyst-3d-tutorial/tree/master/end-of-chapter-projects/empty-game).
