# schulte-grid

schulte-grid 5x5 game.

## build

Use egui and eframe, follow
[emilk/eframe_template](https://github.com/emilk/eframe_template/).

Add `rustflags = ["--cfg", "getrandom_backend=\"wasm_js\""]` and wasm_js feature
for getrandom (according to rustc output)

`ord.py` is use to help me reduce font file `fs.ttf` size.

### Wasm

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly)
and publish it as a web page.

Use [Trunk](https://trunkrs.dev/) to build for web target.

1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will
   rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning
   below.
5. Just run `trunk build --release`. It will generate a `dist` directory as a
   "static html" website

> `assets/sw.js` script will try to cache our app, and loads the cached version
> when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load
> the latest builds during development.
