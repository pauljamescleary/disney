## Render JSON Structure

- `data / StandardCollection / containers` - an array of container
  - I _believe_ each one of these is a "row" to display on the screen
  - `set / text` - the text to display for the row, contains content, language
  - `set / meta` - defines some metdata, including `page_size` which might be useful
  - `items` - where the _bulk_ of the data lives, describes each "cell" within a "row"
    - each `item` is a tile to display in the row
    - `image` - defines image to show in the tile
      - `hero_collection`
        - looks to be a background for this tile
        - `0.71` - has a URL inside to the image
          - `masterWidth` 579
          - `masterHeight` 1055
        - `1.78` - also has a URL inside to the image
          - `masterWidth` 3840
          - `masterHeight` 2160
      - `hero_title`
        - also looks to be a background for this tile
        - Has sizese 1.78, 3.00, 3.91
      - `logo`
        - the actual TEXT on the image that appears
        - For example, for the disney colleciton, this is an image of the words "Disney"
        - Has ONE size 2.00
      - `logo_layer`
        - Contains the logo plus some additional info
        - e.g. adds the word "Collection" and "Streaming only on Disney+"
        - Has sizes 1.78, 3.00, 3.91
      - `tile`
        - Shows the entire tile, appears to be what is used in the instructions?
        - 0.71 - appears to be PORTRAIT
        - 1.78 - appears to be LANDSCAPE


## Requirements

**Parse the json structure, and display each "row"**

1. Pull the `item / text` as the "row title"
2. For each row, parse the `items` collection
   1. Render the `item / tile / 1.78` image for each tile
   2. Initially this is all that is needed

**Respond to KEYBOARD events**

I presume this allows navigating through the screen like using a remote

- UP + DOWN - navigate to the "rows"
- LEFT + RIGHT - navigate among the "tiles" WITHIN a "row"
  
**Focused tile must be scaled up**

I presume this happens when you NAVIGATE to a title, and focus moves to it
Not sure exactly what is meant by "scaled up"

## Extra Credit

Dynamically populate the "ref sets" as they come into view?
- Not sure what this means, if this is tiles, rows, or both?  Perhaps only render those that you can see at first?

Allow interaction or selection of a tile.  For example, show a modal with data on selection.

Incorporate transitions and/or visual aesthetics

Add some Disney Magic
