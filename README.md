# Hill Builder

A simple game about building hills.

## Notes

- [x] [get mouse world position](https://bevyengine.org/examples/3d-rendering/3d-viewport-to-world/)
- [x] issue with ray not selecting mouse over objects
- [x] store row and col grid indicies
- [x] every block clicked will affect it's neighbors, and so forth
- [x] hovered over block gives visual indication
- [x] store block neighbors for shift checking
  - [x] store neighbors after map generate
  - [x] impl a `fn allocate_neighbors()` for `Block`
  - [x] only check neighbors for each shift finished
- [ ] CLI - for in game debugging and testing
- [ ] map generator should have custom and proc-gen
  - [ ] build map and save with CLI
  - [ ] proc-gen should have ranges per desired results

## Resources

- [Mesh-Picking](https://bevyengine.org/examples/picking/mesh-picking/)
