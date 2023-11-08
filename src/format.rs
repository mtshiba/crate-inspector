use rustdoc_types::{Type, GenericArg, GenericArgs, FnDecl, GenericBound, Path, PolyTrait, GenericParamDef, GenericParamDefKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeFormatKind {
    #[default]
    Rust,
    Erg,
}

impl TypeFormatKind {
    pub const fn is_rust(self) -> bool {
        matches!(self, Self::Rust)
    }

    pub const fn is_erg(self) -> bool {
        matches!(self, Self::Erg)
    }

    pub const fn has_lifetime(self) -> bool {
        matches!(self, Self::Rust)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TypeFormatter {
    kind: TypeFormatKind,
}

impl TypeFormatter {
    pub const fn erg() -> Self {
        Self { kind: TypeFormatKind::Erg }
    }

    pub fn and_symbol(&self) -> &str {
        if self.kind.is_rust() {
            " + "
        } else {
            " and "
        }
    }

    pub fn generic_enclosure(&self) -> (&str, &str) {
        if self.kind.is_rust() {
            ("<", ">")
        } else {
            ("(", ")")
        }
    }

    pub fn fn_decl_to_string(&self, decl: &FnDecl) -> String {
        let mut s = String::new();
        s.push('(');
        for (i, (name, input)) in decl.inputs.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(name);
            s.push_str(": ");
            s.push_str(&self.type_to_string(input));
        }
        s.push(')');
        if let Some(output) = &decl.output {
            s.push_str(" -> ");
            s.push_str(&self.type_to_string(output));
        } else if self.kind.is_erg() {
            s.push_str(" -> NoneType");
        }
        s
    }

    pub fn generic_args_to_string(&self, args: &GenericArgs) -> String {
        let mut s = String::new();
        match args {
            GenericArgs::AngleBracketed { args, .. } => {
                if !args.is_empty() {
                    s.push_str(self.generic_enclosure().0);
                }
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&self.generic_arg_to_string(arg));
                }
                if !args.is_empty() {
                    s.push_str(self.generic_enclosure().1);
                }
            }
            GenericArgs::Parenthesized { inputs, output } => {
                s.push('(');
                for (i, input) in inputs.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&self.type_to_string(input));
                }
                s.push(')');
                if let Some(output) = output {
                    s.push_str(" -> ");
                    s.push_str(&self.type_to_string(output));
                }
            }
        }
        s
    }

    pub fn generic_arg_to_string(&self, arg: &GenericArg) -> String {
        match arg {
            GenericArg::Type(ty) => self.type_to_string(ty),
            GenericArg::Lifetime(lifetime) => lifetime.to_string(),
            GenericArg::Infer => "_".to_string(),
            other => todo!("{:?}", other),
        }
    }

    pub fn bound_to_string(&self, bound: &GenericBound) -> String {
        match bound {
            GenericBound::TraitBound { trait_, generic_params, .. } => {
                let mut s = String::new();
                if !generic_params.is_empty() {
                    s.push_str("for<");
                }
                for (i, param) in generic_params.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&self.generic_param_def_to_string(param));
                }
                if !generic_params.is_empty() {
                    s.push_str("> ");
                }
                s.push_str(&self.path_to_string(trait_));
                s
            }
            other => todo!("{:?}", other),
        }
    }

    pub fn poly_trait_to_string(&self, poly_trait: &PolyTrait) -> String {
        let mut s = String::new();
        s.push_str(&self.path_to_string(&poly_trait.trait_));
        for param in poly_trait.generic_params.iter() {
            s.push_str(&self.generic_param_def_to_string(param));
        }
        s
    }

    pub fn generic_param_def_to_string(&self, param: &GenericParamDef) -> String {
        let mut s = String::new();
        s.push_str(&param.name);
        match &param.kind {
            GenericParamDefKind::Lifetime { outlives } => {
                if !outlives.is_empty() {
                    s.push_str(": ");
                    for (i, outlive) in outlives.iter().enumerate() {
                        if i > 0 {
                            s.push_str(self.and_symbol());
                        }
                        s.push_str(outlive);
                    }
                }
            }
            other => todo!("{:?}", other),
        }
        s
    }

    pub fn type_to_string(&self, ty: &Type) -> String {
        match ty {
            Type::Primitive(primitive) if self.kind.is_erg() => match &primitive[..] {
                "bool" => "Bool".to_string(),
                "u8" | "u16" | "u32" | "u64" | "u128" => "Nat".to_string(),
                "i8" | "i16" | "i32" | "i64" | "i128" => "Int".to_string(),
                "f32" | "f64" => "Float".to_string(),
                "char" | "str" | "String" => "Str".to_string(),
                other => other.to_string(),
            },
            Type::Primitive(primitive) => primitive.to_string(),
            Type::ResolvedPath(path) => self.path_to_string(path),
            Type::FunctionPointer(func) => {
                let mut s = String::new();
                if self.kind.is_rust() {
                    s.push_str("fn");
                }
                s.push_str(&self.fn_decl_to_string(&func.decl));
                s
            }
            Type::DynTrait(trait_) => {
                let mut s = String::new();
                if self.kind.is_rust() {
                    s.push_str("dyn ");
                }
                for (i, poly_trait) in trait_.traits.iter().enumerate() {
                    if i > 0 {
                        s.push_str(self.and_symbol());
                    }
                    s.push_str(&self.poly_trait_to_string(poly_trait));
                }
                if self.kind.has_lifetime() {
                    if let Some(life) = &trait_.lifetime {
                        s.push_str(" + ");
                        s.push_str(life);
                    }
                }
                s
            }
            Type::ImplTrait(bounds) => {
                let mut s = String::new();
                if self.kind.is_rust() {
                    s.push_str("impl ");
                }
                for (i, bound) in bounds.iter().enumerate() {
                    if i > 0 {
                        s.push_str(self.and_symbol());
                    }
                    s.push_str(&self.bound_to_string(bound));
                }
                s
            }
            Type::BorrowedRef { lifetime, mutable, type_ } => {
                let mut s = String::new();
                if self.kind.is_rust() {
                    s.push('&');
                    if *mutable {
                        s.push_str("mut ");
                    }
                } else if *mutable {
                    s.push_str("RefMut(");
                } else {
                    s.push_str("Ref(");
                }
                if self.kind.has_lifetime() {
                    if let Some(lifetime) = lifetime {
                        s.push_str(&lifetime.to_string());
                    }
                }
                s.push(' ');
                s.push_str(&self.type_to_string(type_));
                if self.kind.is_erg() {
                    s.push(')');
                }
                s
            }
            Type::Infer => "_".to_string(),
            Type::Generic(gen) => gen.to_string(),
            Type::Slice(type_) => {
                format!("[{}]", self.type_to_string(type_))
            }
            Type::Array { type_, len } => {
                format!("[{}; {}]", self.type_to_string(type_), len)
            }
            Type::QualifiedPath { name, args, self_type, trait_ } => {
                let mut s = String::new();
                if !self.kind.is_erg() {
                    s.push('<');
                }
                s.push_str(&self.type_to_string(self_type));
                if let Some(trait_) = trait_ {
                    if self.kind.is_erg() {
                        s.push_str("|<: ");
                    } else {
                        s.push_str(" as ");
                    }
                    s.push_str(&self.path_to_string(trait_));
                    if self.kind.is_erg() {
                        s.push('|');
                    }
                }
                if self.kind.is_erg() {
                    s.push('.');
                } else {
                    s.push('>');
                    s.push_str("::");
                }
                s.push_str(name);
                s.push_str(&self.generic_args_to_string(args));
                s
            }
            Type::RawPointer { mutable, type_ } => {
                let mut s = String::new();
                s.push('*');
                if *mutable {
                    s.push_str("mut ");
                }
                s.push_str(&self.type_to_string(type_));
                s
            }
            Type::Tuple(types) => {
                let mut s = String::new();
                s.push('(');
                for (i, type_) in types.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&self.type_to_string(type_));
                }
                s.push(')');
                s
            }
        }
    }

    pub fn path_to_string(&self, path: &Path) -> String {
        let mut s = String::new();
        if self.kind.is_erg() {
            s.push_str(&path.name.replace("::", "."));
        } else {
            s.push_str(&path.name);
        }
        if let Some(args) = path.args.as_deref() {
            s.push_str(&self.generic_args_to_string(args));
        }
        s
    }
}
