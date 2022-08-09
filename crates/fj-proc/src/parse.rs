use proc_macro2::Ident;
use syn::{
    bracketed, parenthesized, parse::Parse, parse_quote, Expr, ItemFn,
    ReturnType, Type,
};

/// The call to `fj::register_model!()`.
#[derive(Debug)]
pub(crate) struct Initializer {
    pub(crate) model: Model,
}

/// The generated `Model` struct and its `fj::Model` impl.
#[derive(Debug)]
pub(crate) struct Model {
    pub(crate) metadata: Metadata,
    pub(crate) geometry: GeometryFunction,
}

/// The model metadata we return in `<_ as fj::Model>::metadata()`.
#[derive(Debug)]
pub(crate) struct Metadata {
    pub(crate) name: String,
    pub(crate) arguments: Vec<ArgumentMetadata>,
}

/// Metadata for a specific argument.
#[derive(Debug)]
pub(crate) struct ArgumentMetadata {
    pub(crate) name: String,
    pub(crate) default_value: Option<Expr>,
}

/// The `<_ as fj::Model>::shape()` function.
#[derive(Debug)]
pub(crate) struct GeometryFunction {
    pub(crate) geometry_function: Ident,
    pub(crate) arguments: Vec<ExtractedArgument>,
    pub(crate) constraints: Vec<Constraint>,
    pub(crate) fallible: bool,
}

#[derive(Debug)]
pub(crate) struct ExtractedArgument {
    pub(crate) ident: Ident,
    pub(crate) ty: Type,
    pub(crate) default_value: Option<Expr>,
}

#[derive(Debug)]
pub(crate) struct Constraint {
    pub(crate) target: Ident,
    pub(crate) expr: Expr,
    pub(crate) kind: ConstraintKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ConstraintKind {
    Min,
    Max,
}

pub(crate) fn parse(f: &ItemFn) -> syn::Result<Initializer> {
    let model = parse_model(f)?;

    Ok(Initializer { model })
}

fn parse_model(item: &ItemFn) -> syn::Result<Model> {
    let geometry_function = item.sig.ident.clone();

    let args: Vec<Argument> = item
        .sig
        .inputs
        .iter()
        .map(|inp| parse_quote!(#inp))
        .collect();

    let metadata = Metadata {
        name: geometry_function.to_string(),
        arguments: args
            .iter()
            .map(|a| ArgumentMetadata {
                name: a.ident.to_string(),
                default_value: a.default(),
            })
            .collect(),
    };

    let geometry = GeometryFunction {
        geometry_function,
        arguments: args
            .iter()
            .map(|a| ExtractedArgument {
                ident: a.ident.clone(),
                default_value: a.default(),
                ty: a.ty.clone(),
            })
            .collect(),
        constraints: args.iter().flat_map(argument_constraints).collect(),
        fallible: match &item.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ty) => contains_result(ty),
        },
    };

    Ok(Model { metadata, geometry })
}

fn contains_result(ty: &Type) -> bool {
    match ty {
        Type::Path(p) => p.path.segments.last().unwrap().ident == "Result",
        _ => false,
    }
}

fn argument_constraints(arg: &Argument) -> Vec<Constraint> {
    let attr = match arg.attr.as_ref() {
        Some(a) => a,
        None => return Vec::new(),
    };

    let mut constraints = Vec::new();

    if let Some(min) = attr.get_minimum() {
        constraints.push(Constraint {
            target: arg.ident.clone(),
            expr: min.val,
            kind: ConstraintKind::Min,
        });
    }

    if let Some(max) = attr.get_maximum() {
        constraints.push(Constraint {
            target: arg.ident.clone(),
            expr: max.val,
            kind: ConstraintKind::Max,
        });
    }

    constraints
}

/// Represents one parameter given to the `model`.
///
/// ```text
/// #[param(default=3, min=4)] num_points: u64
/// ^^^^^^^^^^^^^^^^^^^^^^^^^^ ~~~~~~~~~~  ^^^-- ty
///            |                    |
///          attr                 ident
/// ```
#[derive(Debug, Clone)]
struct Argument {
    attr: Option<HelperAttribute>,
    ident: Ident,
    ty: Type,
}

impl Argument {
    fn default(&self) -> Option<Expr> {
        self.attr
            .as_ref()
            .and_then(|attr| attr.get_default())
            .map(|param| param.val)
    }
}

impl Parse for Argument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attr = None;
        if input.peek(syn::token::Pound) {
            attr = Some(input.parse()?);
        }
        let ident: Ident = input.parse()?;

        let _: syn::token::Colon = input.parse()?;

        let ty: Type = input.parse()?;
        Ok(Self { attr, ident, ty })
    }
}

/// Represents all arguments given to the `#[param]` attribute eg:
///
/// ```text
/// #[param(default=3, min=4)]
///         ^^^^^^^^^^^^^^^^
/// ```
#[derive(Debug, Clone)]
struct HelperAttribute {
    param: Option<syn::punctuated::Punctuated<DefaultParam, syn::Token![,]>>,
}

impl Parse for HelperAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr_content;
        let param_content;
        let _: syn::token::Pound = input.parse()?;
        bracketed!(attr_content in input);
        let ident: Ident = attr_content.parse()?;
        if ident != *"param" {
            return Err(syn::Error::new_spanned(
                ident.clone(),
                format!(
                    "Unknown attribute \"{}\" found, expected \"param\"",
                    ident
                ),
            ));
        }

        if attr_content.peek(syn::token::Paren) {
            parenthesized!(param_content in attr_content);
            if param_content.is_empty() {
                Ok(Self { param: None })
            } else {
                Ok(Self {
                param: Some(
                    syn::punctuated::Punctuated::parse_separated_nonempty_with(
                        &param_content,
                        DefaultParam::parse,
                    )?,
                ),
            })
            }
        } else {
            Ok(Self { param: None })
        }
    }
}

impl HelperAttribute {
    fn get_parameter(&self, parameter_name: &str) -> Option<DefaultParam> {
        if let Some(values) = self.param.clone() {
            values.into_iter().find(|val| val.ident == *parameter_name)
        } else {
            None
        }
    }

    fn get_default(&self) -> Option<DefaultParam> {
        self.get_parameter("default")
    }

    fn get_minimum(&self) -> Option<DefaultParam> {
        self.get_parameter("min")
    }

    fn get_maximum(&self) -> Option<DefaultParam> {
        self.get_parameter("max")
    }
}

/// Represents one argument given to the `#[param]` attribute eg:
///
/// ```text
/// #[param(default=3)]
///         ^^^^^^^^^----- is parsed as DefaultParam{ ident: Some(default), val: 3 }
/// ```
#[derive(Debug, Clone)]
struct DefaultParam {
    ident: Ident,
    val: syn::Expr,
}

impl Parse for DefaultParam {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let ident: Ident = input.parse()?;
            let _: syn::token::Eq = input.parse()?;
            Ok(Self {
                ident,
                val: input.parse()?,
            })
        } else {
            Err(input.parse::<Ident>().expect_err("No identifier found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::{quote, ToTokens};

    #[test]
    fn parse_a_typical_model_function() {
        let tokens = quote! {
            pub fn spacer(
                #[param(default = 1.0, min = inner * 1.01)] outer: f64,
                #[param(default = 0.5, max = outer * 0.99)] inner: f64,
                height: f64,
            ) -> fj::Shape {
                let outer_edge = fj::Sketch::from_circle(fj::Circle::from_radius(outer));
                let inner_edge = fj::Sketch::from_circle(fj::Circle::from_radius(inner));

                let footprint = outer_edge.difference(&inner_edge);
                let spacer = footprint.sweep([0., 0., height]);

                spacer.into()
            }
        };
        let function: ItemFn = syn::parse2(tokens).unwrap();

        let Initializer {
            model: Model { metadata, geometry },
        } = parse(&function).unwrap();

        // Note: we can't #[derive(PartialEq)] on our parsed structs because
        // proc_macro2::Ident and friends don't implement PartialEq, so let's
        // manually check everything parsed correctly.
        let Metadata { name, arguments } = metadata;
        assert_eq!(name, "spacer");
        let expected_meta = &[
            ("outer".to_string(), Some("1.0".to_string())),
            ("inner".to_string(), Some("0.5".to_string())),
            ("height".to_string(), None),
        ];
        let meta: Vec<_> = arguments
            .iter()
            .map(|arg| {
                (
                    arg.name.clone(),
                    arg.default_value
                        .as_ref()
                        .map(|v| v.to_token_stream().to_string()),
                )
            })
            .collect();
        assert_eq!(meta, expected_meta);

        let GeometryFunction {
            geometry_function,
            arguments,
            constraints,
            fallible,
        } = geometry;
        assert_eq!(geometry_function.to_string(), "spacer");
        assert!(!fallible);
        let arguments: Vec<_> = arguments
            .iter()
            .map(|arg| {
                (
                    arg.ident.to_string(),
                    arg.default_value
                        .as_ref()
                        .map(|v| v.to_token_stream().to_string()),
                )
            })
            .collect();
        assert_eq!(arguments, expected_meta);
        let expected_constraints = &[
            (
                "outer".to_string(),
                "inner * 1.01".to_string(),
                ConstraintKind::Min,
            ),
            (
                "inner".to_string(),
                "outer * 0.99".to_string(),
                ConstraintKind::Max,
            ),
        ];
        let constraints: Vec<_> = constraints
            .iter()
            .map(|Constraint { kind, expr, target }| {
                (
                    target.to_string(),
                    expr.to_token_stream().to_string(),
                    *kind,
                )
            })
            .collect();
        assert_eq!(constraints, expected_constraints);
    }

    #[test]
    fn parse_fallible_function() {
        let tokens = quote! {
            pub fn spacer() -> Result<fj::Shape, Whatever> {
                todo!()
            }
        };
        let function: ItemFn = syn::parse2(tokens).unwrap();

        let init = parse(&function).unwrap();

        assert!(init.model.geometry.fallible);
    }
}
