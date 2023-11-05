# `crate-inspector`

Inspect the public APIs of Rust library crates

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
