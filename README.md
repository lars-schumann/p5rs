running the server:
```
cd p5rs/
cargo run
```
<br/>
<br/>

editing the wasm (hopefully eventually in the browser):
```
cd p5rs/wasm_canvas/
```
edit files in src/ (functions setup() and draw() are expected to exist by name without any args)
```
cargo install wasm-pack
wasm-pack build --target web --out-dir ../static/wasm
```
