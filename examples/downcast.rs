fn main() {
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
}
