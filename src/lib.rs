use std::ops::Deref;
use std::path::Path;

use rustdoc_types::{Id, Type};

pub trait CrateItem<'a> {
    type Inner;
    fn downcast(inner: &'a rustdoc_types::ItemEnum) -> Option<&'a Self::Inner>;
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, inner: &'a Self::Inner) -> Self;
    fn item(&self) -> &'a rustdoc_types::Item;
    fn inner(&self) -> &'a Self::Inner;
}

pub trait HasType {
    fn type_(&self) -> &Type;
}

pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModuleItem<'a> {
    krate: &'a Crate,
    item: &'a rustdoc_types::Item,
    module: &'a rustdoc_types::Module,
}

impl<'a> CrateItem<'a> for ModuleItem<'a> {
    type Inner = rustdoc_types::Module;
    fn downcast(inner: &'a rustdoc_types::ItemEnum) -> Option<&'a Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Module(module) => Some(module),
            _ => None,
        }
    }
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, module: &'a Self::Inner) -> Self {
        Self {
            krate,
            item,
            module,
        }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.module
    }
}

impl HasName for ModuleItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl<'a> ModuleItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn item_ids(&self) -> impl Iterator<Item = &Id> {
        self.module.items.iter()
    }

    pub fn items(&self) -> impl Iterator<Item = &rustdoc_types::Item> {
        self.item_ids().map(|id| &self.krate.index[id])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FunctionItem<'a> {
    item: &'a rustdoc_types::Item,
    func: &'a rustdoc_types::Function,
}

impl<'a> CrateItem<'a> for FunctionItem<'a> {
    type Inner = rustdoc_types::Function;
    fn downcast(inner: &rustdoc_types::ItemEnum) -> Option<&Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Function(func) => Some(func),
            _ => None,
        }
    }
    fn new(_krate: &'_ Crate, item: &'a rustdoc_types::Item, func: &'a Self::Inner) -> Self {
        Self { item, func }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.func
    }
}

impl HasName for FunctionItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl<'a> FunctionItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn inputs(&self) -> impl Iterator<Item = &(String, Type)> {
        self.func.decl.inputs.iter()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ConstantItem<'a> {
    item: &'a rustdoc_types::Item,
    constant: &'a rustdoc_types::Constant,
}

impl<'a> CrateItem<'a> for ConstantItem<'a> {
    type Inner = rustdoc_types::Constant;
    fn downcast(inner: &'a rustdoc_types::ItemEnum) -> Option<&'a Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Constant(constant) => Some(constant),
            _ => None,
        }
    }
    fn new(_krate: &'_ Crate, item: &'a rustdoc_types::Item, constant: &'a Self::Inner) -> Self {
        Self { item, constant }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.constant
    }
}

impl HasName for ConstantItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl HasType for ConstantItem<'_> {
    fn type_(&self) -> &Type {
        &self.constant.type_
    }
}

impl<'a> ConstantItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn type_(&self) -> &Type {
        &self.constant.type_
    }

    pub fn expr(&self) -> &str {
        &self.constant.expr
    }

    pub fn value(&self) -> Option<&str> {
        self.constant.value.as_deref()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StaticItem<'a> {
    item: &'a rustdoc_types::Item,
    static_: &'a rustdoc_types::Static,
}

impl<'a> CrateItem<'a> for StaticItem<'a> {
    type Inner = rustdoc_types::Static;
    fn downcast(inner: &'a rustdoc_types::ItemEnum) -> Option<&'a Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Static(static_) => Some(static_),
            _ => None,
        }
    }
    fn new(_krate: &'_ Crate, item: &'a rustdoc_types::Item, static_: &'a Self::Inner) -> Self {
        Self { item, static_ }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.static_
    }
}

impl HasName for StaticItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl HasType for StaticItem<'_> {
    fn type_(&self) -> &Type {
        &self.static_.type_
    }
}

impl<'a> StaticItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn type_(&self) -> &Type {
        &self.static_.type_
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StructItem<'a> {
    krate: &'a Crate,
    item: &'a rustdoc_types::Item,
    struct_: &'a rustdoc_types::Struct,
}

impl<'a> CrateItem<'a> for StructItem<'a> {
    type Inner = rustdoc_types::Struct;
    fn downcast(inner: &'a rustdoc_types::ItemEnum) -> Option<&'a Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Struct(struct_) => Some(struct_),
            _ => None,
        }
    }
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, struct_: &'a Self::Inner) -> Self {
        Self {
            krate,
            item,
            struct_,
        }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.struct_
    }
}

impl HasName for StructItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl<'a> StructItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn field_ids(&self) -> Option<impl Iterator<Item = &Id>> {
        match &self.struct_.kind {
            rustdoc_types::StructKind::Unit => None,
            rustdoc_types::StructKind::Tuple(_) => None,
            rustdoc_types::StructKind::Plain { fields, .. } => Some(fields.iter()),
        }
    }

    pub fn fields(&self) -> Option<impl Iterator<Item = FieldItem>> {
        self.field_ids().map(|ids| {
            ids.map(|id| {
                let item = &self.krate.index[id];
                let rustdoc_types::ItemEnum::StructField(field) = &item.inner else {
                    panic!("expected struct field, got {:?}", item.inner);
                };
                FieldItem { item, field }
            })
        })
    }

    pub fn impls(&self) -> impl Iterator<Item = ImplItem> {
        self.krate.impls().filter(|imp| {
            let Type::ResolvedPath(path) = imp.for_() else {
                return false;
            };
            path.id == self.item.id
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FieldItem<'a> {
    item: &'a rustdoc_types::Item,
    field: &'a rustdoc_types::Type,
}

impl<'a> CrateItem<'a> for FieldItem<'a> {
    type Inner = rustdoc_types::Type;
    fn downcast(inner: &'a rustdoc_types::ItemEnum) -> Option<&'a Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::StructField(field) => Some(field),
            _ => None,
        }
    }
    fn new(_krate: &'_ Crate, item: &'a rustdoc_types::Item, field: &'a Self::Inner) -> Self {
        Self { item, field }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.field
    }
}

impl<'a> HasType for FieldItem<'a> {
    fn type_(&self) -> &Type {
        self.field
    }
}

impl HasName for FieldItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl<'a> FieldItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn type_(&self) -> &Type {
        self.field
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TraitItem<'a> {
    krate: &'a Crate,
    item: &'a rustdoc_types::Item,
    trait_: &'a rustdoc_types::Trait,
}

impl<'a> CrateItem<'a> for TraitItem<'a> {
    type Inner = rustdoc_types::Trait;
    fn downcast(inner: &'a rustdoc_types::ItemEnum) -> Option<&'a Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Trait(trait_) => Some(trait_),
            _ => None,
        }
    }
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, trait_: &'a Self::Inner) -> Self {
        Self {
            krate,
            item,
            trait_,
        }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.trait_
    }
}

impl HasName for TraitItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl<'a> TraitItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn item_ids(&self) -> impl Iterator<Item = &Id> {
        self.trait_.items.iter()
    }

    pub fn items(&self) -> impl Iterator<Item = &rustdoc_types::Item> {
        self.item_ids().map(|id| &self.krate.index[id])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EnumItem<'a> {
    krate: &'a Crate,
    item: &'a rustdoc_types::Item,
    enum_: &'a rustdoc_types::Enum,
}

impl<'a> CrateItem<'a> for EnumItem<'a> {
    type Inner = rustdoc_types::Enum;
    fn downcast(inner: &rustdoc_types::ItemEnum) -> Option<&Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Enum(enum_) => Some(enum_),
            _ => None,
        }
    }
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, enum_: &'a Self::Inner) -> Self {
        Self { krate, item, enum_ }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.enum_
    }
}

impl HasName for EnumItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl<'a> EnumItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn variant_ids(&self) -> impl Iterator<Item = &Id> {
        self.enum_.variants.iter()
    }

    pub fn variants(&self) -> impl Iterator<Item = VariantItem> {
        self.variant_ids().map(|id| {
            let item = &self.krate.index[id];
            let rustdoc_types::ItemEnum::Variant(variant) = &item.inner else {
                panic!("expected variant, got {:?}", item.inner);
            };
            VariantItem { item, variant }
        })
    }

    pub fn impls(&self) -> impl Iterator<Item = ImplItem> {
        self.krate.impls().filter(|imp| {
            let Type::ResolvedPath(path) = imp.for_() else {
                return false;
            };
            path.id == self.item.id
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VariantItem<'a> {
    item: &'a rustdoc_types::Item,
    variant: &'a rustdoc_types::Variant,
}

impl<'a> CrateItem<'a> for VariantItem<'a> {
    type Inner = rustdoc_types::Variant;
    fn downcast(inner: &rustdoc_types::ItemEnum) -> Option<&Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Variant(variant) => Some(variant),
            _ => None,
        }
    }
    fn new(_krate: &'_ Crate, item: &'a rustdoc_types::Item, variant: &'a Self::Inner) -> Self {
        Self { item, variant }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.variant
    }
}

impl HasName for VariantItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl<'a> VariantItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn discriminant(&self) -> Option<&rustdoc_types::Discriminant> {
        self.variant.discriminant.as_ref()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ImplItem<'a> {
    krate: &'a Crate,
    item: &'a rustdoc_types::Item,
    impl_: &'a rustdoc_types::Impl,
}

impl<'a> CrateItem<'a> for ImplItem<'a> {
    type Inner = rustdoc_types::Impl;
    fn downcast(inner: &rustdoc_types::ItemEnum) -> Option<&Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Impl(impl_) => Some(impl_),
            _ => None,
        }
    }
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, impl_: &'a Self::Inner) -> Self {
        Self { krate, item, impl_ }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.impl_
    }
}

impl<'a> ImplItem<'a> {
    pub fn id(&self) -> &Id {
        &self.item.id
    }

    pub fn item_ids(&self) -> impl Iterator<Item = &Id> {
        self.impl_.items.iter()
    }

    pub fn items(&self) -> impl Iterator<Item = &rustdoc_types::Item> {
        self.item_ids().map(|id| &self.krate.index[id])
    }

    pub fn constants(&self) -> impl Iterator<Item = ConstantItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Constant(constant) => Some(ConstantItem { item, constant }),
            _ => None,
        })
    }

    pub fn functions(&self) -> impl Iterator<Item = FunctionItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Function(func) => Some(FunctionItem { item, func }),
            _ => None,
        })
    }

    pub fn trait_(&self) -> Option<&rustdoc_types::Path> {
        self.impl_.trait_.as_ref()
    }

    pub fn for_(&self) -> &rustdoc_types::Type {
        &self.impl_.for_
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MacroItem<'a> {
    item: &'a rustdoc_types::Item,
    macro_: &'a String,
}

impl<'a> CrateItem<'a> for MacroItem<'a> {
    type Inner = String;
    fn downcast(inner: &rustdoc_types::ItemEnum) -> Option<&Self::Inner> {
        match inner {
            rustdoc_types::ItemEnum::Macro(macro_) => Some(macro_),
            _ => None,
        }
    }
    fn new(_krate: &'_ Crate, item: &'a rustdoc_types::Item, macro_: &'a Self::Inner) -> Self {
        Self { item, macro_ }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.macro_
    }
}

impl HasName for MacroItem<'_> {
    fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }
}

impl<'a> MacroItem<'a> {
    pub fn name(&self) -> &str {
        self.item.name.as_ref().unwrap()
    }

    pub fn macro_(&self) -> &str {
        self.macro_
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crate(rustdoc_types::Crate);

impl Deref for Crate {
    type Target = rustdoc_types::Crate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Crate {
    pub fn items(&self) -> impl Iterator<Item = &rustdoc_types::Item> {
        self.0.index.values()
    }

    pub fn downcast<'a, T: CrateItem<'a> + 'a>(
        &'a self,
        item: &'a rustdoc_types::Item,
    ) -> Option<T> {
        let inner = T::downcast(&item.inner)?;
        Some(T::new(self, item, inner))
    }

    pub fn modules(&self) -> impl Iterator<Item = ModuleItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Module(module) => Some(ModuleItem {
                krate: self,
                item,
                module,
            }),
            _ => None,
        })
    }

    pub fn functions(&self) -> impl Iterator<Item = FunctionItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Function(func) => Some(FunctionItem { item, func }),
            _ => None,
        })
    }

    pub fn constants(&self) -> impl Iterator<Item = ConstantItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Constant(constant) => Some(ConstantItem { item, constant }),
            _ => None,
        })
    }

    pub fn statics(&self) -> impl Iterator<Item = StaticItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Static(static_) => Some(StaticItem { item, static_ }),
            _ => None,
        })
    }

    pub fn structs(&self) -> impl Iterator<Item = StructItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Struct(struct_) => Some(StructItem {
                krate: self,
                item,
                struct_,
            }),
            _ => None,
        })
    }

    pub fn traits(&self) -> impl Iterator<Item = TraitItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Trait(trait_) => Some(TraitItem {
                krate: self,
                item,
                trait_,
            }),
            _ => None,
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = EnumItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Enum(enum_) => Some(EnumItem {
                krate: self,
                item,
                enum_,
            }),
            _ => None,
        })
    }

    pub fn impls(&self) -> impl Iterator<Item = ImplItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Impl(impl_) => Some(ImplItem {
                krate: self,
                item,
                impl_,
            }),
            _ => None,
        })
    }

    pub fn macros(&self) -> impl Iterator<Item = MacroItem> {
        self.items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Macro(macro_) => Some(MacroItem { item, macro_ }),
            _ => None,
        })
    }
}

#[derive(Debug)]
pub enum BuildCrateError {
    RustdocJson(rustdoc_json::BuildError),
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl std::fmt::Display for BuildCrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildCrateError::RustdocJson(err) => err.fmt(f),
            BuildCrateError::Io(err) => err.fmt(f),
            BuildCrateError::Serde(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for BuildCrateError {}

impl From<rustdoc_json::BuildError> for BuildCrateError {
    fn from(err: rustdoc_json::BuildError) -> Self {
        Self::RustdocJson(err)
    }
}

impl From<std::io::Error> for BuildCrateError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for BuildCrateError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

#[derive(Default)]
pub struct CrateBuilder {
    builder: rustdoc_json::Builder,
}

impl CrateBuilder {
    pub fn new() -> Self {
        Self {
            builder: rustdoc_json::Builder::default(),
        }
    }

    pub fn toolchain(mut self, toolchain: impl Into<String>) -> Self {
        self.builder = self.builder.toolchain(toolchain);
        self
    }

    pub fn manifest_path(mut self, manifest_path: impl AsRef<Path>) -> Self {
        self.builder = self.builder.manifest_path(manifest_path);
        self
    }

    pub fn build(self) -> Result<Crate, BuildCrateError> {
        let path = self.builder.build()?;
        let krate = serde_json::from_reader(std::fs::File::open(path)?).map(Crate)?;
        Ok(krate)
    }
}
