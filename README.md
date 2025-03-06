# Hill Builder

A simple game about building hills.

## User Stories

- [ ] user should be able to adjust ground up and down
  - [ ] user cannot adjust ground when water is present
- [ ] user needs to move water indirectly to win level
- [ ] user can reset level easily

## Notes

- [x] [get mouse world position](https://bevyengine.org/examples/3d-rendering/3d-viewport-to-world/)
- [x] issue with ray not selecting mouse over objects
- [x] store row and col grid indicies
- [x] every block clicked will affect it's neighbors, and so forth
- [x] hovered over block gives visual indication
- [x] camera controls - need to move around, rotate around, and zoom
- [x] store block neighbors for shift checking
  - [x] store neighbors after map generate
  - [x] impl a `fn allocate_neighbors()` for `Block`
  - [x] only check neighbors for each shift finished
- [x] CLI - in game console dev tool
  - [x] map gen
  - [x] file storage
  - [ ] testing
- [x] map generator should have custom and proc-gen
  - [x] build map and save with CLI
  - [x] proc-gen should allow raw input
  - [ ] proc-gen should have types per desired results
- [x] water (should be it's own component, separate from the blocks)
- [ ] water movement
  - [x] events
    - [x] ground adjustments -> trigger water check
    - [x] water check (if needs water) -> trigger adjustment
      - [x] get water
      - [x] create water
    - [x] water adjustment -> sets new level
  - [ ] systems
    - [ ] check neighbor compares neighbor water levels
      - [ ] if no neighbor, water check
      - [ ] if yes neighbor, set relative drain rate
    - [x] move water and mesh
    - [x] despawn when no water
- [ ] dev tools should be contained in their own plugin
- [ ] add UI using `bevy_lunex`
- [ ] make a few basic levels
- [ ] QoL
  - [ ] bound camera translation - edge cannot pass screen center
  - [ ] zoom should have more than 2 fixed points

## Stretch

- [ ] rainfall - maps should have rainfall
  - [ ] frequency: the percentage determining whether or not rain is falling
  - [ ] amount: the amount of rain falling when it does raining
- [ ] different block types
  - [ ] sand: 1 layer separation amount
  - [ ] dirt: 2 layer separation amount
  - [ ] clay: 3 layer separation amount
  - [ ] rock: 4 layer separation amount
- [ ] shader code - with big maps, will need greater efficiency
- [ ] ecosystems - will occur automatically given terrain and rainfall conditions
  - [ ] swamp: flatlands, 1 layer deep of water, rains often and a lot
  - [ ] forest: usually hilly geology, rains often, and a fair amount
  - [ ] grasslands: flat geology, rains more than plains
  - [ ] plains: flat or hilly land, rains sometimes and a little
  - [ ] deserts: geology, rarely rains, but floods quickly when it does
- [ ] geologies - the shape of the land
  - [ ] hills: raised layers of clay
  - [ ] mountain: raised layers of rock
  - [ ] lake: 2+ layers of stagnent water
  - [ ] swamp: 1 layer deep of stagnent water
  - [ ] river: connected tiles of flowing water
- [ ] results - automatic output for a given ecological and geological input
  - [ ] swamp - frogs, cranes, cat grass, willows

## Resources

- [Mesh-Picking](https://bevyengine.org/examples/picking/mesh-picking/)
- [bevy-console](https://github.com/RichoDemus/bevy-console)
- [clap](https://docs.rs/clap/latest/clap/)
- [serde](https://docs.rs/serde/latest/serde/)
- [bevy_lunex](https://docs.rs/bevy_lunex/latest/bevy_lunex/)
