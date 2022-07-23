use proc_macro::TokenStream;
use quote::quote;
use syn::{
    bracketed, parenthesized, parse::Parse, parse_macro_input, parse_quote,
};

/// Define a function-based model.
///
/// The simplest model function takes no parameters and returns a hard-coded
/// `fj::Shape`.
///
/// ```rust
/// # use fj_proc::model;
/// use fj::{Circle, Sketch, Shape};
/// #[model]
/// fn model() -> Shape {
///     let circle = Circle::from_radius(10.0);
///     Sketch::from_circle(circle).into()
/// }
/// ```
///
/// For convenience, you can also return anything that could be converted into
/// a `fj::Shape` (e.g. a `fj::Sketch`).
///
/// ```rust
/// # use fj_proc::model;
/// use fj::{Circle, Sketch};
/// #[model]
/// fn model() -> Sketch {
///     let circle = Circle::from_radius(10.0);
///     Sketch::from_circle(circle)
/// }
/// ```
///
/// The return type is checked at compile time. That means something like this
/// won't work because `()` can't be converted into a `fj::Shape`.
///
/// ```rust,compile_fail
/// # use fj_proc::model;
/// #[model]
/// fn model() { todo!() }
/// ```
///
/// The model function's arguments can be anything that implement
/// [`std::str::FromStr`].
///
/// ```rust
/// # use fj_proc::model;
/// #[model]
/// fn cylinder(height: f64, label: String, is_horizontal: bool) -> fj::Shape { todo!() }
/// ```
///
/// Constraints and default values can be added to an argument using the
/// `#[param]` attribute.
///
/// ```rust
/// use fj::syntax::*;
///
/// #[fj::model]
/// pub fn spacer(
///     #[param(default = 1.0, min = inner * 1.01)] outer: f64,
///     #[param(default = 0.5, max = outer * 0.99)] inner: f64,
///     #[param(default = 1.0)] height: f64,
/// ) -> fj::Shape {
///     let outer_edge = fj::Sketch::from_circle(fj::Circle::from_radius(outer));
///     let inner_edge = fj::Sketch::from_circle(fj::Circle::from_radius(inner));
///
///     let footprint = outer_edge.difference(&inner_edge);
///     let spacer = footprint.sweep([0., 0., height]);
///
///     spacer.into()
/// }
/// ```
#[proc_macro_attribute]
pub fn model(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemFn);
    let inputs = &item.sig.inputs;

    let args: Vec<Argument> =
        inputs.iter().map(|inp| parse_quote!(#inp)).collect();

    let mut parameter_extraction = Vec::new();

    let mut min_checks = Vec::new();
    let mut max_checks = Vec::new();

    for Argument { attr, ident, ty } in &args {
        if let Some(attr) = attr {
            if let Some(default) = attr.get_default() {
                let def = default.val;
                parameter_extraction.push(quote! {
                    let #ident: #ty = args.get(stringify!(#ident))
                            .map(|arg| arg.parse().unwrap())
                            .unwrap_or(#def);
                });
            } else {
                parameter_extraction.push(quote! {
                let #ident: #ty = args.get(stringify!(#ident))
                        .map(|arg| arg.parse().unwrap())
                        .expect(format!("A value for `{}` has to be provided since no default is specified",stringify!(#ident)).as_str());
            });
            }

            if let Some(minimum) = attr.get_minimum() {
                let min = minimum.val;
                min_checks.push(quote! {
                if #ident < #min {
                    panic!("Value of `{}` must not be smaller than: {}",stringify!(#ident), #min);
                }
            });
            }
            if let Some(maximum) = attr.get_maximum() {
                let max = maximum.val;
                max_checks.push(quote! {
                if #ident > #max {
                    panic!("Value of `{}` must not be larger than: {}", stringify!(#ident), #max);
                }
            })
            }
        } else {
            parameter_extraction.push(quote! {
                let #ident: #ty = args.get(stringify!(#ident))
                        .map(|arg| arg.parse().unwrap())
                        .expect(format!("A value for `{}` has to be provided since no default is specified",stringify!(#ident)).as_str());
            });
        }
    }

    let function_boilerplate = quote! {
        #[no_mangle]
            pub extern "C" fn model(
                args: &std::collections::HashMap<String, String>
            ) -> fj::Shape
    };

    let function_name = &item.sig.ident;
    let body = &item.block;
    let arg_names: Vec<_> = args.iter().map(|a| &a.ident).collect();
    let arg_types: Vec<_> = args.iter().map(|a| &a.ty).collect();
    let return_type = &item.sig.output;

    quote! {
        #function_boilerplate {
            #(
                #parameter_extraction
            )*
            #(
                #min_checks
            )*
            #(
                #max_checks
            )*

            fn #function_name(
                #( #arg_names : #arg_types ),*
            ) #return_type {
                #body
            }

            #function_name(#( #arg_names),*).into()
        }
    }
    .into()
}

/// Represents one parameter given to the `model`
/// `#[param(default=3, min=4)] num_points: u64`
/// `^^^^^^^^^^^^^^^^^^^^^^^^^^ ~~~~~~~~~~  ^^^-- ty`
/// `           |                    |`
/// `         attr                 ident`
#[derive(Debug, Clone)]
struct Argument {
    pub attr: Option<HelperAttribute>,
    pub ident: proc_macro2::Ident,
    pub ty: proc_macro2::Ident,
}

impl Parse for Argument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attr = None;
        if input.peek(syn::token::Pound) {
            attr = Some(input.parse()?);
        }
        let ident: proc_macro2::Ident = input.parse()?;

        let _: syn::token::Colon = input.parse()?;

        let ty: proc_macro2::Ident = input.parse()?;
        Ok(Self { attr, ident, ty })
    }
}

/// Represents all arguments given to the `#[param]` attribute eg:
/// `#[param(default=3, min=4)]`
/// `        ^^^^^^^^^^^^^^^^`
#[derive(Debug, Clone)]
struct HelperAttribute {
    pub param:
        Option<syn::punctuated::Punctuated<DefaultParam, syn::Token![,]>>,
}

impl Parse for HelperAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr_content;
        let param_content;
        let _: syn::token::Pound = input.parse()?;
        bracketed!(attr_content in input);
        let ident: proc_macro2::Ident = attr_content.parse()?;
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

    pub fn get_default(&self) -> Option<DefaultParam> {
        self.get_parameter("default")
    }

    pub fn get_minimum(&self) -> Option<DefaultParam> {
        self.get_parameter("min")
    }

    pub fn get_maximum(&self) -> Option<DefaultParam> {
        self.get_parameter("max")
    }
}

/// Represents one argument given to the `#[param]` attribute eg:
/// `#[param(default=3)]`
/// `        ^^^^^^^^^----- is parsed as DefaultParam{ ident: Some(default), val: 3 }`
#[derive(Debug, Clone)]
struct DefaultParam {
    pub ident: proc_macro2::Ident,
    pub val: syn::Expr,
}

impl Parse for DefaultParam {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let ident: proc_macro2::Ident = input.parse()?;
            let _: syn::token::Eq = input.parse()?;
            Ok(Self {
                ident,
                val: input.parse()?,
            })
        } else {
            Err(input
                .parse::<proc_macro2::Ident>()
                .expect_err("No identifier found"))
        }
    }
}
