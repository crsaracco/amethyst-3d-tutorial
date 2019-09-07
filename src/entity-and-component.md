# Entity and Component

*[TODO: maybe re-write this a little better -- suggestions welcome]*

*[TODO: also maybe move this page to the Appendix section?]*

We're almost ready to create shapes for our game and actually *draw* something, but first we should understand a little bit about what's going on under the hood in Amethyst when you "create" something in your game world.

At its core, **Amethyst is an [entity-component-system](https://en.wikipedia.org/wiki/Entity_component_system) (ECS) style game engine**. Although ECS is *kind of* a paradigm of creating "objects", it's quite different from the "[object-oriented programming](https://en.wikipedia.org/wiki/Object-oriented_programming)" (OOP) style that's commonly taught in schools.

An **entity** represents a single object in your game world. To the programmer, these are typically unnamed; i.e. you don't "bind" them to a variable name, the ECS framework will store them for you.

Under the hood, an entity is just a number representing its entity ID (and a little bit of extra data for internal ECS use which you don't have to worry about yet). You can't tell the game engine anything else about the entity, like *"what kind of object is this?"* or *"how do I render this entity?"* or anything else. It's just a number, that you don't even store yourself.

So how do you use entities to build up objects for a game?

**Components** allow you to store additional information about an entity, like *"this is a monster"*, *"this object is red"*, or *"this object is located at (35.0, 1.0, 0.0)"*.

In ECS, each different type of component is stored in its own storage. Let's say you had some RPG game -- all of the HPs for all of the different monsters and players in the game would be stored in some "HP storage"; all the MPs in their own "MP storage", all of the items in their own "Item storage", and so on.

In OOP, you would store all of the data about your monsters in one struct per monster (its HP, its MP, a pointer to its mesh/texture/etc, its location in the world, etc).

The reason for this entity/component split in ECS is that it allows you to store and retrieve your objects more efficiently, with noticeable effects on game performance. For example, if you cast some spell that did X damage to *every* monster in the world:

 - In OOP, you'd have to load *all* of the structs for *all* of the monsters in the game. This would be terrible for cache performance, because you're pulling in a bunch of data you're ultimately not going to be using right now.
 - In ECS, you just have to load the "HP storage" for all of the monsters -- you're using all of the data you just loaded, and *only* that data.

In Amethyst's flavor of ECS, each instance of a component modifies exactly one entity. Each entity doesn't know anything about which components modify it. Here's a little example:

```
Entities   PositionComponent       TypeComponent   HpComponent
0          [0] (1.0, 2.0, 0.0)     [0] Player      [0] 100
1          [1] (5.0, 3.0, 0.0)     [1] Monster     [1] 50
2          [2] (10.0, -2.0, 0.0)   [2] Building    [3] 75
3          [3] (0.0, 5.0, 0.0)     [3] Monster
                                    ^  ^
                      (entity number)  (component value)
```

There are four entities in this tiny game: one player, one building, and two monsters. The positions for each entity are given in the table. The player (entity 0) has 100 HP, while the two monsters (entities 1 and 3) have 50 and 75 HP. Note that the building doesn't have any HP; every component is optional for each entity, and it doesn't really make sense for a building to have HP, so it doesn't have any `HpComponents` attached to it.

If you were just given `entity 2`, you wouldn't be able to know anything about it without scanning all of your component storages. Luckily, you never need to go from entity -> component, because you don't really do things this way in ECS.

One neat thing about ECS is it allows you to do "joins" which are kind of similar to how relational databases work. If you want to modify all of the monsters' HPs, you would do a join on `TypeComponent` and `HpComponent` (which means that any entity that doesn't have both a type and an HP will be automatically filtered out), then you go through the results of your join and update the HP data if the Type is "Monster". If you also gave each monster a `ColorComponent`, you could join on (`TypeComponent`, `ColorComponent`, and `HpComponent`), and update only the `HP`s of the entities that are both `Monster` and `Blue`. You can start to imagine some pretty cool functionality this enables you to do that would be pretty messy (and perhaps inefficient) in an OOP-style game engine!

## Entities and components in Amethyst

To declare a component in Amethyst, you simply create a data structure (either a `struct` or an `enum`) that will hold the information that the component conveys, then you implement the `Component` trait for that data structure.

```rust
use amethyst::ecs::{Component, DenseVecStorage};

// Type component
enum Type {
    Player,
    Monster,
    Building,
}

impl Component for Type {
    type Storage = DenseVecStorage<Self>;
}

// HP component
struct Hp {
    hp: u32,
}

impl Component for Hp {
    type Storage = DenseVecStorage<Self>;
}
```

Remember that each component has its own storage -- the type that you give `type Storage` tells Amethyst how you want to actually store those components in memory. There are [a few different types of storage to choose from](https://docs.rs/amethyst/0.9.0/amethyst/ecs/prelude/index.html?search=Storage), but `DenseVecStorage` is a good default type to use if you don't have any good reasons to use something else.

Usually, when you create an entity, you also give it any components that describe it. It'll end up looking something like this:

```rust
world.create_entity()   // Create a new entity
    .with(Type::Player) // Create a Type component, and associate it with this entity)
    .with(Hp{hp: 100})  // Create an HP component, and associate it with this entity)
    .build();
```

where "world" is the thing that keeps track of all your entities and components (and other things, which we'll see later).

## More reading

You can also read up on Entity and Component at [the Amethyst book's page on this topic](https://book.amethyst.rs/stable/concepts/entity_and_component.html).
