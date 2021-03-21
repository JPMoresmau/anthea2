# Anthea's quest

Anthea's quest (not a very imaginative name, I'll grant you that) is a tiny tiny game written in Rust, using the Bevy framework (https://github.com/bevyengine/bevy). It's not intended to be a full game, but I wanted to practise both Rust and ECS programming, and play around with some ideas for role-playing/adventure game.

## Game principles

You control one character, and like in most RPGs there are "talents" that represent the skills your character possesses and improve with time, an inventory, magic spells. However, some things I tried are:
- items tend to be either quest items or consumable items
    - quest items go into your inventory, allow you do to specific actions, and disappear when not needed any more
        - example here are the scissors, that you can use in three different ways, and disappear when used
    - consumable items do not appear in your inventory and usually just impact your talents
        - even things usually not considered consumable are, like weapons: walking over the sword consumes it, increasing your weapon talent
- fights are not automatic, you get to choose if you want to fight the rats or not
- along with that, interacting with characters/monsters/affordances shows you the possible actions and will tell you if more actions would be possible, so you can search for alternatives.

I tried these things because inventory management is not (for me) the most exiting thing in games, and I wanted to deprioritize fighting (there's ample room to experiment with RPGs that don't require you to kill every bug you encounter to level up).

## Development principles

The game uses Rust and the Bevy engine (the current main branch, so it may not work when the Bevy code changes), so we rely a lot on ECS. Points of note are:
- I implemented my own tileset handling because the Bevy compatible libraries that can handle tiled files where not working at the time I looked at them with the version of Bevy I was using
- I store a lot of information about the game state in Resources: Inventory, Journal, Spells, Talents, etc. are all resources
- There are a lot of different game states, maybe too much and it could be simplified
- I've done my own system for UI (messages and menu) and maybe there are higher level primitives I could use

## Limitations

- The game is very short, there's only one little area
- There is only one save slot, but given the previous point, you can do the whole game in two minutes, so the save/load functionality was more for me to tackle this awkward (and that could probably be made a lot better) section
- I've just used free assets, and it shows I'm not an artist

## Assets

- Tiles and sprites by Dungeon Crawl Stone Soup (http://crawl.develz.org/wordpress/), licensed under Creative Commons
- UI elements by RPG GUI contruction kit v1.0 (https://opengameart.org/content/rpg-gui-construction-kit-v10), licensed under CC-BY 3.0
- Sounds by Kenney Vleugels (https://www.kenney.nl), licensed under Creative Commons
- Font GRECOromanLubedWrestling by Timmy Wakefield (https://www.urbanfonts.com/fonts/GRECOromanLubedWrestling.font)

Thanks to all for the great work and giving these assets for free!