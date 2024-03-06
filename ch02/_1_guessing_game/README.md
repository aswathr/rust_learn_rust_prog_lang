### Guessing game
* `crate` dependencies are specified under `[dependencies]` in the `Cargo.toml` file
* Semantic versioning: `"0.8.5"` (`"major.minor.patch"`) is actually >= 0.8.5 and < 0.9.0 (Public APIs are assumed to work with patch versions)
* Building without changes will result in Rust NOT rebuilding that project/ crate
* Check in the `Cargo.lock` to maintain crate versions until we explicitly update the version
* To update all crates to the latest versions fitting the specifications, run `cargo update`
