# `crate-inspector`

Inspect the public APIs of Rust library crates

## Prerequisite

This crate requires that you have the nightly Rust toolchain installed, as it uses the latest `rustdoc` API (and does not require compiling with the nightly toolchain).

## Examples

### Getting items

```rust
use crate_inspector::CrateBuilder;

let builder = CrateBuilder::default()
    .toolchain("nightly")
    .manifest_path("Cargo.toml");
let krate = builder.build().unwrap();

for item in krate.items() {
    println!("item: {:?}", item.name);
}
for strc in krate.structs() {
    println!("struct: {}", strc.name());
    println!("#impls: {}", strc.impls().count());
}
for enm in krate.enums() {
    println!("enum: {}", enm.name());
    println!("variants: {:?}", enm.variants().collect::<Vec<_>>());
    println!("#methods: {}", enm.impls().fold(0, |acc, i| acc + i.functions().count()));
}
for sub in krate.sub_modules() {
    println!("submodule: {}", sub.name());
}
if let Some(foo) = krate.get_item("foo") {
    println!("id: {:?}", foo.id);
}
```

### Downcasting

```rust
use crate_inspector::{CrateBuilder, StructItem};

let builder = CrateBuilder::default()
    .toolchain("nightly")
    .manifest_path("Cargo.toml");
let krate = builder.build().unwrap();

for item in krate.items() {
    if let Some(strc) = krate.downcast::<StructItem>(item) {
        println!("struct: {}", strc.name());
    }
}
```

## Compatibility

This crate depends on [rustdoc's public API](https://github.com/rust-lang/rustdoc-types).

Here is a table mapping crate-inspector versions to corresponding rustdoc versions:

| crate-inspector | rustdoc-types |
|-----------------|---------|
| 0.1             | 0.23    |
| 0.2             | 0.32    |
| 0.3             | 0.54    |
| 0.4             | 0.55    |

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
