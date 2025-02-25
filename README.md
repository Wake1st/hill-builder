# Hill Builder

A simple game about building hills.

## Notes

- [x] [get mouse world position](https://bevyengine.org/examples/3d-rendering/3d-viewport-to-world/)
- [x] issue with ray not selecting mouse over objects
- [x] store row and col grid indicies
- [x] every block clicked will affect it's neighbors, and so forth
- [ ] camera controls - need to move around, rotate around, and zoom
- [ ] shader code - with big maps, will need greater efficiency
- [ ] different block types
  - [ ] water: flows downhill at some set rate
  - [ ] sand: 1 layer separation amount
  - [ ] dirt: 2 layer separation amount
  - [ ] clay: 3 layer separation amount
  - [ ] rock: 4 layer separation amount
- [ ] rainfall - maps should have rainfall
  - [ ] frequency: the percentage determining whether or not rain is falling
  - [ ] amount: the amount of rain falling when it does raining
- [ ] ecosystems - will occur automatically given terrain conditions
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
