Immediate TO-DO in order of priority:
- [x] Cache flatmap converted images
- [x] Process land mask to have black filled in as transparent, and pass to renderer
- [ ] Process provinces.png, pass province bounding boxes and ids to renderer and render the bounding boxes
- [ ] Grab history info about who owns what province to pass with the province info

Low Priority TO-DO:
- [ ] Color in land mask and flatmap overlay properly.
- [ ] Parallelize image processing

Known Bugs:
- If the map images load in the wrong order, they will layer in the wrong order
Known Issues:
- Land mask and flatmap overlay are not colored properly.
