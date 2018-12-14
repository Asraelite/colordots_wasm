# WASM Force

Particle simulator.

### Setup

Install Rustup with your package manager, then get the nightly wasm32-unknown-unknown target.

`rustup toolchain install nightly`  
`rustup target add wasm32-unknown-unknown --toolchain nightly`

Also install `wasm-gc` so smaller WASM binaries can be generated.

`cargo install --git https://github.com/alexcrichton/wasm-gc`

Install some static HTTP server. I use the Rust crate `https`:

`cargo install https`

If you use a different server, change the command from `http` to whatever you use in the top-level file `serve`.

Run `./build` to build the client and then `./serve` to serve the files on a HTTP server.
