Immediate TO-DO in order of priority:
- [x] Cache flatmap converted images
- [x] Process land mask to have black filled in as transparent, and pass to renderer
- [x] Process provinces.png, pass province bounding boxes and ids to renderer and render the bounding boxes
- [x] Render state bounding boxes
- [x] Render country bounding boxes
- [x] Click on country to reveal its states
- [x] Transfer selected state to selected country
- [x] Click on state to reveal its provinces
- [x] Transfer selected province to selected country
- [x] Persist state and province transfers as pdx script
- [ ] Allow opening of working directory, save state and province transfer data there

Low Priority TO-DO:
- [ ] Color in land mask and flatmap overlay properly.
- [ ] Parallelize image processing
- [ ] Look into caching and cache busting to speed up load times
- [x] ~~Look into diagonal vector detection to optimize geojson rendering (Implemented)~~ (Reverted)
- [ ] Make a proper readme

Known Bugs:
- If the map images load in the wrong order, they will layer in the wrong order
- ~~Some provinces seem to be stuck in infinite loops when tracing their geojson bounds~~ (squished)
- Some province borders do not trace properly, so they may be slightly inaccurate. No visual quirks though.

Known Issues:
- Land mask and flatmap overlay are not colored properly.
- ~~Internal geojson bounds need to be accounted for, this is especially noticeable with sea provinces overlapping islands.~~ (Resolved!)
- Parsing the provinces and states is very slow. Should seek to optimize this and cache.
- If state history file is change, border changes are not picked up unless state cache is manually busted
