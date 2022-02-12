# Adunis

v0.1.5

## Current Plans

- Menu
- Self-host server (needs networking abstracted from end user preferably), including chat system
  - Chat system must have option to only have rolls and no text chat, as text chat is usually just abused in the context
- Area designer (e.g. to make a marketplace map) (This will likely require a lot more effort in graphic design for assets than programming)
- Continental generator (generates a map of the world, similar to [Azgaar's Fantasy Map Generator](https://azgaar.github.io/Fantasy-Map-Generator/))
  - Should be able to store lore in certain sections of the map
  - Should be able to mark certain things as not including in export, so DM can export the map to players so they can see the world while the DM can store story elements without spoiling
- Character creation system (similar to [Dungeon Master's Vault](https://www.dungeonmastersvault.com/)), this should include:
  - Ability to have custom rulesets, not *just* D&D5e
  - Ability to import custom player-made content reliably
  - Prompt players for all things they need to create their character
- Integrated rolling system
  - The rolls should be able to be taken from the character creation system 
  - Rolls should be able to be done by commands manually if needed
  - Rolls should be able to be put on a macro which is easy to setup and use
  - All rolls done from macros and CCS should be able to be done both for only the roller to see, and for it to be automatically given in the chat system
- ~~Character icon creator, similar to [Hero Forge](www.heroforge.com) but very focussed~~
  <!-- Maybe come back to this at a later point. -->

## Current Stack

GUI: [egui](https://github.com/emilk/egui)

(Undecided) Networking: Tokio/Axum/Poem/Actix
