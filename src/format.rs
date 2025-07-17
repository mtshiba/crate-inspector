use rustdoc_types::{
    FunctionSignature, GenericArg, GenericArgs, GenericBound, GenericParamDef, GenericParamDefKind,
    Path, PolyTrait, Type,
};

pub fn fn_sig_to_string(decl: &FunctionSignature) -> String {
    let mut s = String::new();
    s.push('(');
    for (i, (name, input)) in decl.inputs.iter().enumerate() {
        if i > 0 {
            s.push_str(", ");
        }
        s.push_str(name);
        s.push_str(": ");
        s.push_str(&type_to_string(input));
    }
    s.push(')');
    if let Some(output) = &decl.output {
        s.push_str(" -> ");
        s.push_str(&type_to_string(output));
    }
    s
}

pub fn generic_args_to_string(args: &GenericArgs) -> String {
    let mut s = String::new();
    match args {
        GenericArgs::AngleBracketed { args, .. } => {
            if !args.is_empty() {
                s.push('<');
            }
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                s.push_str(&generic_arg_to_string(arg));
            }
            if !args.is_empty() {
                s.push('>');
            }
        }
        GenericArgs::Parenthesized { inputs, output } => {
            s.push('(');
            for (i, input) in inputs.iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                s.push_str(&type_to_string(input));
            }
            s.push(')');
            if let Some(output) = output {
                s.push_str(" -> ");
                s.push_str(&type_to_string(output));
            }
        }
        GenericArgs::ReturnTypeNotation => {
            s.push_str("(..)")
        }
    }
    s
}

pub fn generic_arg_to_string(arg: &GenericArg) -> String {
    match arg {
        GenericArg::Type(ty) => type_to_string(ty),
        GenericArg::Lifetime(lifetime) => lifetime.to_string(),
        GenericArg::Infer => "_".to_string(),
        other => todo!("{:?}", other),
    }
}

pub fn bound_to_string(bound: &GenericBound) -> String {
    match bound {
        GenericBound::TraitBound {
            trait_,
            generic_params,
            ..
        } => {
            let mut s = String::new();
            if !generic_params.is_empty() {
                s.push_str("for<");
            }
            for (i, param) in generic_params.iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                s.push_str(&generic_param_def_to_string(param));
            }
            if !generic_params.is_empty() {
                s.push_str("> ");
            }
            s.push_str(&path_to_string(trait_));
            s
        }
        other => todo!("{:?}", other),
    }
}

pub fn poly_trait_to_string(poly_trait: &PolyTrait) -> String {
    let mut s = String::new();
    s.push_str(&path_to_string(&poly_trait.trait_));
    for param in poly_trait.generic_params.iter() {
        s.push_str(&generic_param_def_to_string(param));
    }
    s
}

pub fn generic_param_def_to_string(param: &GenericParamDef) -> String {
    let mut s = String::new();
    s.push_str(&param.name);
    match &param.kind {
        GenericParamDefKind::Lifetime { outlives } => {
            if !outlives.is_empty() {
                s.push_str(": ");
                for (i, outlive) in outlives.iter().enumerate() {
                    if i > 0 {
                        s.push_str(" + ");
                    }
                    s.push_str(outlive);
                }
            }
        }
        GenericParamDefKind::Type {
            bounds, default, ..
        } => {
            if !bounds.is_empty() {
                s.push_str(": ");
                for (i, bound) in bounds.iter().enumerate() {
                    if i > 0 {
                        s.push_str(" + ");
                    }
                    s.push_str(&bound_to_string(bound));
                }
            }
            if let Some(default) = default {
                s.push_str(" = ");
                s.push_str(&type_to_string(default));
            }
        }
        other => todo!("{:?}", other),
    }
    s
}

pub fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Primitive(primitive) => primitive.to_string(),
        Type::ResolvedPath(path) => path_to_string(path),
        Type::FunctionPointer(func) => {
            let mut s = String::new();
            s.push_str("fn");
            s.push_str(&fn_sig_to_string(&func.sig));
            s
        }
        Type::DynTrait(trait_) => {
            let mut s = String::new();
            s.push_str("dyn ");
            for (i, poly_trait) in trait_.traits.iter().enumerate() {
                if i > 0 {
                    s.push_str(" + ");
                }
                s.push_str(&poly_trait_to_string(poly_trait));
            }
            if let Some(life) = &trait_.lifetime {
                s.push_str(" + ");
                s.push_str(life);
            }
            s
        }
        Type::ImplTrait(bounds) => {
            let mut s = String::new();
            s.push_str("impl ");
            for (i, bound) in bounds.iter().enumerate() {
                if i > 0 {
                    s.push_str(" + ");
                }
                s.push_str(&bound_to_string(bound));
            }
            s
        }
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let mut s = String::new();
            s.push('&');
            if *is_mutable {
                s.push_str("mut ");
            }
            if let Some(lifetime) = lifetime {
                s.push_str(&lifetime.to_string());
            }
            s.push(' ');
            s.push_str(&type_to_string(type_));
            s
        }
        Type::Infer => "_".to_string(),
        Type::Generic(gen) => gen.to_string(),
        Type::Slice(type_) => {
            format!("[{}]", type_to_string(type_))
        }
        Type::Array { type_, len } => {
            format!("[{}; {}]", type_to_string(type_), len)
        }
        Type::QualifiedPath {
            name,
            args,
            self_type,
            trait_,
        } => {
            let mut s = String::new();
            s.push('<');
            s.push_str(&type_to_string(self_type));
            if let Some(trait_) = trait_ {
                s.push_str(" as ");
                s.push_str(&path_to_string(trait_));
            }
            s.push('>');
            s.push_str("::");
            s.push_str(name);
            if let Some(args) = args {
                s.push_str(&generic_args_to_string(args));
            }
            s
        }
        Type::RawPointer { is_mutable, type_ } => {
            let mut s = String::new();
            s.push('*');
            if *is_mutable {
                s.push_str("mut ");
            }
            s.push_str(&type_to_string(type_));
            s
        }
        Type::Tuple(types) => {
            let mut s = String::new();
            s.push('(');
            for (i, type_) in types.iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                s.push_str(&type_to_string(type_));
            }
            s.push(')');
            s
        }
        Type::Pat {
            type_,
            __pat_unstable_do_not_use,
        } => type_to_string(type_),
    }
}

pub fn path_to_string(path: &Path) -> String {
    let mut s = String::new();
    s.push_str(&path.path);
    if let Some(args) = path.args.as_deref() {
        s.push_str(&generic_args_to_string(args));
    }
    s
}
