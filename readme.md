# Bevy + WASM

## Intro
This is a demo for the snake game using [Bevy 0.13](https://bevyengine.org/) following [Creating a Snake Clone in Rust, with Bevy](https://mbuffett.com/posts/bevy-snake-tutorial/) blog post. In addition, I compiled the project to run with WASM. 

> If you want to do the same, fork the repo and follow the next section.

## Info
### Bevy
[Bevy](https://bevyengine.org/) is a game engine in [Rust](https://www.rust-lang.org/es) that uses Entity Component System pattern to develope video games.

### WebAssembly
[WebAssembly](https://webassembly.org/) is an amazing tool, basically, it lets you create a virtual machine to compile bineries for web. You can choose to have it for client or server side, this give you a lot of flexibility and power to create applications in a new way. 

## Build for WASM
You can get more info from the [Unofficial Bevy Cheat Book | WASM section](https://bevy-cheatbook.github.io/platforms/wasm.html).

First, you need to modify your `toml` file, add the next profiles:
```toml
[profile.release]
opt-level = 'z' # or opt-level = 's'
# For the biggest improvements at the cost of the slowest compile times:
lto = true
codegen-units = 1
```

> `opt-level` is the optimisation you want: `z` for size and `s` for speed.

Then install the compilation target: 
```sh
rustup target install wasm32-unknown-unknown 
```

Now you can build the project:
```sh
cargo build --release --target wasm32-unknown-unknown
```

It takes a while to build, once is done, a new directory appear: `./target/wasm32-unknown-unknown/release`. In there is the wasm file (you can test it by running an http server in there). Now you can prepare the project for a GitHub Page manually or with the next section.

### (Optional) Use wasm-bindgen CLI for GitHub pages
With [GitHub Pages](https://pages.github.com/), you can host client side websites and the wasm-bindgen CLI tool assists you preparing the files to run the game in the client side.

First, install the tool
```sh
cargo install wasm-bindgen-cli
```
Then `cd` into the directory with your wasm file and create the output directory: `mkdir out`.

Finally, you can run:

```sh 
mkdir out && wasm-bindgen --no-typescript --target web \
--out-dir ./out/ \
--out-name "mygame" \
./mygame.wasm
```

If you don't get the html file, you can create a `index.html` file with:
```html
<!doctype html>
<html lang="en">

<body style="margin: 0px;">
  <script type="module">
    import init from './mygame.js'

    init().catch((error) => {
      if (!error.message.startsWith("Using exceptions for control flow, don't mind me. This isn't actually an error!")) {
        throw error;
      }
    });
  </script>
</body>

</html>
```

Now you can branch those files to its own branch and enable GH Pages in your repo, or can follow [this chapter](https://bevy-cheatbook.github.io/platforms/wasm/gh-pages.html).

### (Optional) WASM optimization with binaryen toolkit
After previous steps, you can use the [binaryen]() wasm-opt to further down wasm file size:
``` sh
# Optimize for size (z profile).
wasm-opt -Oz -o output.wasm input.wasm

# Optimize for size (s profile).
wasm-opt -Os -o output.wasm input.wasm

# Optimize for speed.
wasm-opt -O3 -o output.wasm input.wasm

# Optimize for both size and speed.
wasm-opt -O -ol 100 -s 100 -o output.wasm input.wasm
```

>Remember to rename the output same as the original or update the `.js` file with the output wasm file name.

### (Optional) Development
For development, running the WASM compilation target can be achived by installing:
```sh
cargo install wasm-server-runner
```
and then create a target (inside the project) `./cargo/config.toml` and adding to the file:
```toml
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
```
>Alternatively, add env var:  
`export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner
`


With either of the previous config, you can run with that target:
```sh
cargo run --target wasm32-unknown-unknown
```
