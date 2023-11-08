pub mod format;

use std::ops::Deref;
use std::path::Path;

use rustdoc_types::{Id, Type};

pub trait CrateItem<'a> {
    type Inner;
    fn downcast(inner: &'a rustdoc_types::ItemEnum) -> Option<&'a Self::Inner>;
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, inner: &'a Self::Inner) -> Self;
    fn krate(&self) -> &'a Crate;
    fn item(&self) -> &'a rustdoc_types::Item;
    fn inner(&self) -> &'a Self::Inner;
    fn is_crate_item(&self) -> bool {
        self.item().crate_id == 0
    }
    fn is_root_item(&self) -> bool {
        self.module().is_some_and(|module| module.id() == &self.krate().root)
    }
    fn is_external_item(&self) -> bool {
        self.item().crate_id != 0
    }
    fn id(&'a self) -> &'a Id {
        &self.item().id
    }
    fn module(&self) -> Option<ModuleItem<'a>> {
        self.krate().all_modules()
            .find(|module| module.module.items.contains(&self.item().id))
    }
}

pub trait HasType {
    fn type_(&self) -> &Type;
}

pub trait HasName {
    fn name(&self) -> &str;
}

macro_rules! impl_items {
    ($ty: ident < $l: lifetime >) => {
        impl<$l> $ty<$l> {
            pub fn constants(&self) -> impl Iterator<Item = ConstantItem> {
                self.items().filter_map(|item| match &item.inner {
                    rustdoc_types::ItemEnum::Constant(constant) => Some(ConstantItem { krate: self.krate, item, constant }),
                    _ => None,
                })
            }

            pub fn functions(&self) -> impl Iterator<Item = FunctionItem> {
                self.items().filter_map(|item| match &item.inner {
                    rustdoc_types::ItemEnum::Function(func) => Some(FunctionItem { krate: self.krate, item, func }),
                    _ => None,
                })
            }

            pub fn structs(&self) -> impl Iterator<Item = StructItem> {
                self.items().filter_map(|item| match &item.inner {
                    rustdoc_types::ItemEnum::Struct(struct_) => Some(StructItem {
                        krate: self.krate,
                        item,
                        struct_,
                    }),
                    _ => None,
                })
            }

            pub fn enums(&self) -> impl Iterator<Item = EnumItem> {
                self.items().filter_map(|item| match &item.inner {
                    rustdoc_types::ItemEnum::Enum(enum_) => Some(EnumItem {
                        krate: self.krate,
                        item,
                        enum_,
                    }),
                    _ => None,
                })
            }

            pub fn traits(&self) -> impl Iterator<Item = TraitItem> {
                self.items().filter_map(|item| match &item.inner {
                    rustdoc_types::ItemEnum::Trait(trait_) => Some(TraitItem {
                        krate: self.krate,
                        item,
                        trait_,
                    }),
                    _ => None,
                })
            }

            pub fn impls(&self) -> impl Iterator<Item = ImplItem> {
                self.items().filter_map(|item| match &item.inner {
                    rustdoc_types::ItemEnum::Impl(impl_) => Some(ImplItem {
                        krate: self.krate,
                        item,
                        impl_,
                    }),
                    _ => None,
                })
            }
        }
    };
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
    fn krate(&self) -> &'a Crate {
        self.krate
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

impl_items!(ModuleItem <'a>);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FunctionItem<'a> {
    krate: &'a Crate,
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
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, func: &'a Self::Inner) -> Self {
        Self { krate, item, func }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.func
    }
    fn krate(&self) -> &'a Crate {
        self.krate
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

    pub fn is_method(&self) -> bool {
        self.func.decl.inputs.first().is_some_and(|(name, _)| name == "self")
    }

    pub fn is_associated(&self) -> bool {
        self.krate.all_impls()
            .any(|imp| imp.item_ids().any(|id| id == &self.item.id))
    }

    pub fn associated_impl(&self) -> Option<ImplItem<'a>> {
        self.krate.all_impls()
            .find(|imp| imp.item_ids().any(|id| id == &self.item.id))
    }

    pub fn inputs(&self) -> impl Iterator<Item = &(String, Type)> {
        self.func.decl.inputs.iter()
    }

    pub fn output(&self) -> Option<&Type> {
        self.func.decl.output.as_ref()
    }

    pub fn decl(&self) -> &rustdoc_types::FnDecl {
        &self.func.decl
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ConstantItem<'a> {
    krate: &'a Crate,
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
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, constant: &'a Self::Inner) -> Self {
        Self { krate, item, constant }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.constant
    }
    fn krate(&self) -> &'a Crate {
        self.krate
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
    krate: &'a Crate,
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
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, static_: &'a Self::Inner) -> Self {
        Self { krate, item, static_ }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.static_
    }
    fn krate(&self) -> &'a Crate {
        self.krate
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
    fn krate(&self) -> &'a Crate {
        self.krate
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
                FieldItem { krate: self.krate, item, field }
            })
        })
    }

    pub fn impls(&self) -> impl Iterator<Item = ImplItem> {
        self.krate.all_impls().filter(|imp| {
            let Type::ResolvedPath(path) = imp.for_() else {
                return false;
            };
            path.id == self.item.id
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FieldItem<'a> {
    krate: &'a Crate,
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
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, field: &'a Self::Inner) -> Self {
        Self { krate, item, field }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.field
    }
    fn krate(&self) -> &'a Crate {
        self.krate
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
    fn krate(&self) -> &'a Crate {
        self.krate
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
    fn krate(&self) -> &'a Crate {
        self.krate
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
            VariantItem { krate: self.krate, item, variant }
        })
    }

    pub fn impls(&self) -> impl Iterator<Item = ImplItem> {
        self.krate.all_impls().filter(|imp| {
            let Type::ResolvedPath(path) = imp.for_() else {
                return false;
            };
            path.id == self.item.id
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VariantItem<'a> {
    krate: &'a Crate,
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
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, variant: &'a Self::Inner) -> Self {
        Self { krate, item, variant }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.variant
    }
    fn krate(&self) -> &'a Crate {
        self.krate
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
    fn krate(&self) -> &'a Crate {
        self.krate
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

    pub fn trait_(&self) -> Option<&rustdoc_types::Path> {
        self.impl_.trait_.as_ref()
    }

    pub fn for_(&self) -> &rustdoc_types::Type {
        &self.impl_.for_
    }
}

impl_items!(ImplItem <'a>);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MacroItem<'a> {
    krate: &'a Crate,
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
    fn new(krate: &'a Crate, item: &'a rustdoc_types::Item, macro_: &'a Self::Inner) -> Self {
        Self { krate, item, macro_ }
    }
    fn item(&self) -> &'a rustdoc_types::Item {
        self.item
    }
    fn inner(&self) -> &'a Self::Inner {
        self.macro_
    }
    fn krate(&self) -> &'a Crate {
        self.krate
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

#[derive(Clone, PartialEq, Eq)]
pub struct Crate(rustdoc_types::Crate);

impl std::fmt::Debug for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Crate")
            .field("root", &self.root)
            .field("crate_version", &self.crate_version)
            .field("...", &"...")
            .finish()
    }
}

impl Deref for Crate {
    type Target = rustdoc_types::Crate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Crate {
    /// All items in the crate, including external items referenced locally.
    pub fn all_items(&self) -> impl Iterator<Item = &rustdoc_types::Item> {
        self.0.index.values()
    }

    /// Items in the crate, excluding external items referenced locally.
    pub fn items(&self) -> impl Iterator<Item = &rustdoc_types::Item> {
        self.all_items().filter(|&item| item.crate_id == 0)
    }

    pub fn krate(&self) -> &Crate {
        self
    }

    pub fn item_summary(&self) -> impl Iterator<Item = &rustdoc_types::ItemSummary> {
        self.0.paths.values()
    }

    /// Downcast an item to a specific type `T: CrateItem`.
    pub fn downcast<'a, T: CrateItem<'a> + 'a>(
        &'a self,
        item: &'a rustdoc_types::Item,
    ) -> Option<T> {
        let inner = T::downcast(&item.inner)?;
        Some(T::new(self, item, inner))
    }

    pub fn all_modules(&self) -> impl Iterator<Item = ModuleItem> {
        self.all_items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Module(module) => Some(ModuleItem {
                krate: self,
                item,
                module,
            }),
            _ => None,
        })
    }

    /// root module included
    pub fn modules(&self) -> impl Iterator<Item = ModuleItem> {
        self.all_modules().filter(|module| module.is_crate_item())
    }

    /// root module not included
    pub fn sub_modules(&self) -> impl Iterator<Item = ModuleItem> {
        self.all_modules().filter(|module| module.item.id != self.root)
    }

    /// Enumerates all functions including submodules.
    /// methods & associated functions & function declarations included
    pub fn all_functions(&self) -> impl Iterator<Item = FunctionItem> {
        self.all_items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Function(func) => Some(FunctionItem { krate: self, item, func }),
            _ => None,
        })
    }

    /// Enumerates root module functions.
    /// methods & associated functions & function declarations not included
    pub fn functions(&self) -> impl Iterator<Item = FunctionItem> {
        self.all_functions().filter(|func| func.is_root_item() && !func.is_method() && !func.is_associated() && func.func.has_body)
    }

    /// Enumerates all constants including submodules
    pub fn all_constants(&self) -> impl Iterator<Item = ConstantItem> {
        self.all_items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Constant(constant) => Some(ConstantItem { krate: self, item, constant }),
            _ => None,
        })
    }

    /// Enumerates root module constants
    pub fn constants(&self) -> impl Iterator<Item = ConstantItem> {
        self.all_constants().filter(|constant| constant.is_root_item())
    }

    /// Enumerates all statics including submodules
    pub fn all_statics(&self) -> impl Iterator<Item = StaticItem> {
        self.all_items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Static(static_) => Some(StaticItem { krate: self, item, static_ }),
            _ => None,
        })
    }

    /// Enumerates root module statics
    pub fn statics(&self) -> impl Iterator<Item = StaticItem> {
        self.all_statics().filter(|static_| static_.is_root_item())
    }

    /// Enumerates all structs including submodules
    pub fn all_structs(&self) -> impl Iterator<Item = StructItem> {
        self.all_items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Struct(struct_) => Some(StructItem {
                krate: self,
                item,
                struct_,
            }),
            _ => None,
        })
    }

    /// Enumerates root module structs
    pub fn structs(&self) -> impl Iterator<Item = StructItem> {
        self.all_structs().filter(|struct_| struct_.is_root_item())
    }

    /// Enumerates all traits including submodules
    pub fn all_traits(&self) -> impl Iterator<Item = TraitItem> {
        self.all_items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Trait(trait_) => Some(TraitItem {
                krate: self,
                item,
                trait_,
            }),
            _ => None,
        })
    }

    /// Enumerates root module traits
    pub fn traits(&self) -> impl Iterator<Item = TraitItem> {
        self.all_traits().filter(|trait_| trait_.is_root_item())
    }

    /// Enumerates all enums including submodules
    pub fn all_enums(&self) -> impl Iterator<Item = EnumItem> {
        self.all_items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Enum(enum_) => Some(EnumItem {
                krate: self,
                item,
                enum_,
            }),
            _ => None,
        })
    }

    /// Enumerates root module enums
    pub fn enums(&self) -> impl Iterator<Item = EnumItem> {
        self.all_enums().filter(|enum_| enum_.is_root_item())
    }

    /// Enumerates all impls including submodules
    pub fn all_impls(&self) -> impl Iterator<Item = ImplItem> {
        self.all_items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Impl(impl_) => Some(ImplItem {
                krate: self,
                item,
                impl_,
            }),
            _ => None,
        })
    }

    /// Enumerates root module impls
    pub fn impls(&self) -> impl Iterator<Item = ImplItem> {
        self.all_impls().filter(|imp| imp.is_root_item())
    }

    /// Enumerates all macros including submodules
    pub fn all_macros(&self) -> impl Iterator<Item = MacroItem> {
        self.all_items().filter_map(|item| match &item.inner {
            rustdoc_types::ItemEnum::Macro(macro_) => Some(MacroItem { krate: self, item, macro_ }),
            _ => None,
        })
    }

    /// Enumerates root module macros
    pub fn macros(&self) -> impl Iterator<Item = MacroItem> {
        self.all_macros().filter(|macro_| macro_.is_root_item())
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

    pub fn all_features(mut self, all_features: bool) -> Self {
        self.builder = self.builder.all_features(all_features);
        self
    }

    pub fn package(mut self, package: impl AsRef<str>) -> Self {
        self.builder = self.builder.package(package);
        self
    }

    pub fn features(mut self, features: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.builder = self.builder.features(features);
        self
    }

    pub fn no_default_features(mut self, no_default_features: bool) -> Self {
        self.builder = self.builder.no_default_features(no_default_features);
        self
    }

    pub fn target(mut self, target: String) -> Self {
        self.builder = self.builder.target(target);
        self
    }

    pub fn target_dir(mut self, target_dir: impl AsRef<Path>) -> Self {
        self.builder = self.builder.target_dir(target_dir);
        self
    }

    pub fn build(self) -> Result<Crate, BuildCrateError> {
        let path = self.builder.build()?;
        let krate = serde_json::from_reader(std::fs::File::open(path)?).map(Crate)?;
        Ok(krate)
    }
}
