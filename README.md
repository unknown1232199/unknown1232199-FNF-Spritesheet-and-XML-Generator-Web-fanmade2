# FNF Spritesheet & XML Generator Web

A browser-based Friday Night Funkin' modding tool for building character spritesheets, XML atlases, animation previews and icon grids. This project is a web adaptation of [UncertainProd's original Spritesheet and XML Maker](https://github.com/UncertainProd/FnF-Spritesheet-and-XML-Maker).

**Live project:** https://uncertainprod.github.io/FNF-Spritesheet-XML-generator-Web/

## Workspace

- **Spritesheet generator:** add individual PNG frames or import an existing PNG + XML atlas.
- **Frame selection:** select individual frames, select all, select a range, clone or delete selected frames.
- **Animation tools:** assign animation prefixes, preview animations and adjust playback FPS and scale.
- **XML editor:** inspect frame metadata, adjust offsets and dimensions, scale and flip frames, and apply expressions to multiple selected frames.
- **Atlas settings:** configure padding, transparent-bound clipping and animation-name prefix rules.
- **Exports:** generate a packed spritesheet + XML archive, or export a PNG sequence.
- **Icongrid generator:** create a new icon grid or use the legacy grid workflow.

The interface includes responsive layouts, dark/light themes and basic/advanced workspace density options. Image processing runs locally in the browser; selected files are not uploaded to a server.

## Getting started

1. Open the live project in a modern browser (Chrome or Firefox recommended).
2. Enter a character name. This name is used for exported files.
3. Choose **Spritesheet Generator** or **Icongrid Generator**.
4. Add your PNG frames, or import an existing spritesheet and its XML.
5. Review the frames, set animation prefixes and adjust XML values as needed.
6. Generate the atlas archive or a PNG sequence.

### Notes about atlas packing

The spritesheet packer trims unused space from the outer right and bottom edges of the generated atlas. Transparent frame clipping can also be configured in Settings. Keep clipping disabled if you need to preserve the original transparent frame bounds.

Large images use browser memory while being processed. For smoother results, avoid loading extremely large source atlases or too many high-resolution frames at once.

## Build from source

Requirements:
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Node.js and npm](https://nodejs.org/en/download)
- wasm-pack

From the svelte-frontend directory:

    npm install
    npm run dev

Create a production build with:

    npm run build

## Credits

- Original web implementation and Rust/WebAssembly image packer: [UncertainProd](https://github.com/UncertainProd/FNF-Spritesheet-XML-generator-Web)
- Original desktop tool: [FnF-Spritesheet-and-XML-Maker](https://github.com/UncertainProd/FnF-Spritesheet-and-XML-Maker)

This fork keeps the upstream project structure and credits. See [LICENSE](LICENSE) for license details.
