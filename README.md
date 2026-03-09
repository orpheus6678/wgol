# wgol: game of life, in webassembly

a rewrite of conways game of life harnessing the blazing fast speeds of rust powered webassembly.
using `wasm-bindgen` to interface rust code compiled to webassembly, with javascript.

## about conways game of life

[the game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) is an evolution simulation
requiring no further input than its initial state. the rules of "life" are thus:

- any live cell with fewer than two live neighbors dies, as if by underpopulation
- any live cell with two or three live neighbors lives on to the next generation
- any live cell with more than three live neighbors dies, as if by overpopulation
- any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction

## steps to install

get [`git`](https://git-scm.com) (optional). install [`wasm-bindgen`](https://github.com/wasm-bindgen/wasm-bindgen). install npm or [pnpm](https://pnpm.io/installation). then follow the steps below:

```bash
# use git or just download this archive from github manually and unpack it
git clone https://github.com/orpheus6678/wgol.git

# change working directory to the local repository
cd wgol

# build the webassembly module using wasm-pack (builds for release by default)
wasm-pack build

# change to 'www' directory and do npm or pnpm install
cd www
pnpm install

# execute the start script using npm or pnpm
pnpm run start
```

that's it. enjoy!

## known issues

the frame drops when relying on the `web_sys::console::log_*` family of functions. as such, logging functionality is conditionally compiled for dev builds only, using the `debug_assertions` attribute.

## planned features

- [ ] option to generate a random universe
- [ ] control the number of generations (ticks) per frame
- [ ] select from common initial universes
- [ ] more aggressive optimizations
