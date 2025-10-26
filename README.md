# Craft Show Entry Cards

A tool that started out as the simple idea to automate the changing of the year on the entry card PDF for [Bourne End Junior Craft Show](https://www.bejuniorcraftshow.co.uk/).

That turned into re-designing the entire card in [Typst](https://typst.app/), having previously been made in Adobe Photoshop.

Of course then the idea came to give users the ability to generate their own entry card PDF with contact details and entrant name/age pre-filled
so they don't need to write out the same information for every entry - only the things that actually do change between entries - which was fairly
easily achieved by having a simple leptos app to take the user inputs and trigger a typst render of the PDF on the client side, avoiding any hosting
costs and preserving data privacy for the users.

[Try it out!](https://mikecroall.github.io/craft-show-entry-cards/)

## Development

### Client-side pre-filled PDF generator webpage
To run a local (slowly) hot-reloading server and automatically open a browser tab to the right local address (using [Trunk](https://trunkrs.dev/)), run:
```bash
trunk serve --open
```

### Typst template
The Typst file can be worked on separately with *much* faster hot-reloading (using [Typst CLI](https://github.com/typst/typst#installation)) by running:
```bash
typst watch entry-card.typ --input=entrants_name="Some Name Here" --input=entrants_age="12"
```
and opening the entry-card.pdf that it writes to disk.

Note the `--input=key=value` items are optional, and more can be added - one for each key that the typst file checks for in the inputs object!

### GitHub Pages deployment
The GitHub Pages deployment is automatically performed by any push to the default branch if the build, test, lint, and audit CI passes.
