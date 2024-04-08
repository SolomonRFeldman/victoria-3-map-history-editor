Immediate TO-DO in order of priority:
- [x] Cache flatmap converted images
- [x] Process land mask to have black filled in as transparent, and pass to renderer
- [x] Process provinces.png, pass province bounding boxes and ids to renderer and render the bounding boxes
- [ ] Render state bounding boxes
- [ ] Grab history info about who owns what province to pass with the province info

Low Priority TO-DO:
- [ ] Color in land mask and flatmap overlay properly.
- [ ] Parallelize image processing
- [ ] Look into caching and cache busting to speed up load times
- [ ] Look into diagonal vector detection to optimize geojson rendering
- [ ] Make a proper readme

Known Bugs:
- If the map images load in the wrong order, they will layer in the wrong order
- Some provinces seem to be stuck in infinite loops when tracing their geojson bounds

Known Issues:
- Land mask and flatmap overlay are not colored properly.
- Internal geojson bounds need to be accounted for, this is especially noticeable with sea provinces overlapping islands.
- Rendering the provinces is very slow to parse, and laggy when rendered. Should seek to only render state bounding boxes next, perhaps also cache the processing of provinces.png
