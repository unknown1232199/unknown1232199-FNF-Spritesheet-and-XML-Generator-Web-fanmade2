# FNF Spritesheet and XML Generator Web — Fan Version (Fantake)

**Fantake** is an independent fan version of the original *FNF Spritesheet and XML Generator Web*, continued as a browser-based toolkit for Friday Night Funkin' mod creators. Build character spritesheets, edit XML atlases, preview animations and create icon grids in one workspace.

> This project is an independent fan-made continuation built from the open-source work of [UncertainProd](https://github.com/UncertainProd/FNF-Spritesheet-XML-generator-Web). Original credits and license notices are preserved below.

## Features

- **Spritesheet generator:** add PNG frames or import an existing PNG + XML atlas.
- **Frame tools:** select, clone, delete, scale, flip and adjust frames.
- **Animation preview:** assign prefixes and preview animations with adjustable FPS and scale.
- **XML editor:** inspect frame metadata, edit offsets and dimensions, and apply changes to selected frames.
- **Atlas settings:** configure padding and transparent-bound clipping.
- **Exports:** generate a packed spritesheet + XML archive or export a PNG sequence.
- **Icongrid generator:** create and edit icon grids.
- **Workspace:** responsive interface, dark/light themes and basic/advanced density options.

Image processing runs locally in your browser. Selected files are not uploaded to a server.

## Run locally

Requirements: Node.js + npm, Rust toolchain, wasm-pack.

```bash
cd svelte-frontend
npm ci
npm run dev
```

Production build:

```bash
npm run build
```

The production site is generated in svelte-frontend/public.

## Deployment

The GitHub Actions workflow builds the Svelte frontend and publishes svelte-frontend/public to GitHub Pages on pushes to main. In repository settings, set Pages → Build and deployment → Source to GitHub Actions.

## Credits and licenses

- Original web implementation and Rust/WebAssembly image packer: [UncertainProd](https://github.com/UncertainProd/FNF-Spritesheet-XML-generator-Web).
- Original desktop tool: [FnF-Spritesheet-and-XML-Maker](https://github.com/UncertainProd/FnF-Spritesheet-and-XML-Maker).
- The upstream project license is included in LICENSE; Rust dependencies retain their respective notices in rust-img-lib/.

Fantake is a fan-made project and is not affiliated with the creators of Friday Night Funkin'.