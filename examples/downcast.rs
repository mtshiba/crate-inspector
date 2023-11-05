use crate_inspector::StructItem;

fn main() {
    let builder = crate_inspector::CrateBuilder::default()
        .toolchain("nightly")
        .manifest_path("Cargo.toml");
    let krate = builder.build().unwrap();

    for item in krate.items() {
        if let Some(strc) = krate.downcast::<StructItem>(item) {
            println!("struct: {}", strc.name());
        }
    }
}
