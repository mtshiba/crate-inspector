#[test]
fn test() {
    use crate_inspector::CrateBuilder;

    let builder = CrateBuilder::default()
        .toolchain("nightly")
        .manifest_path("Cargo.toml");
    let krate = builder.build().unwrap();

    assert_eq!(krate.sub_modules().count(), 1);
    assert_eq!(krate.structs().count(), 14);
    assert_eq!(krate.enums().count(), 1);
    assert_eq!(krate.functions().count(), 0);
    assert_eq!(krate.traits().count(), 3);
}
