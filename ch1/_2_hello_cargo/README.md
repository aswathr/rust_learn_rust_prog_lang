
### hello cargo
cargo is build system and package manager<br>
`cargo new <name_of_package>` creates a new project with `main.rs` in the `src` dir and a `Cargo.toml` file that describes the project and any dependencies it has<br>
`cargo build` builds and spits out the binary if the project compiles in `target/debug`<br>
`cargo check` checks if the project is compilable<br>
`cargo run` builds and runs the binary output in a single go<br>

`cargo build --release` creates a release binary, which is slower to compile but faster to run compared to the `debug` build in `target/release`