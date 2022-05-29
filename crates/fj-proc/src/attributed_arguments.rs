use proc_macro::TokenStream;
use quote::quote;
use syn::{
    bracketed, parenthesized, parse::Parse, parse_macro_input, parse_quote,
};

pub fn attributed_arguments(
    _default_values: TokenStream,
    input: TokenStream,
) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemFn);
    let inputs = item.clone().sig.inputs;
    let args: Vec<Argument> =
        inputs.iter().map(|inp| parse_quote!(#inp)).collect();
    let mut defaults = Vec::new();
    let mut mins = Vec::new();
    let mut maxs = Vec::new();
    let mut names = Vec::new();
    let mut types = Vec::new();
    for arg in args {
        let mut default = None;
        let mut min = None;
        let mut max = None;
        names.push(arg.ident);
        types.push(arg.ty);
        for value in arg.attr.values.clone() {
            if let Some(ident) = value.ident.clone() {
                match ident.to_string().as_str() {
                    "default" => default = Some(value.val.clone()),
                    "min" => min = Some(value.val.clone()),
                    "max" => max = Some(value.val.clone()),
                    _ => {}
                }
            } else {
                default = Some(value.val.clone());
            }
        }
        let attr_param = AttributeParameter::new(default, min, max);
        defaults.push(attr_param.default);
        mins.push(attr_param.min);
        maxs.push(attr_param.max);
    }
    let block = item.block;
    println!(
        "{}",
        quote! {
            #[no_mangle]
            pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
                #(
                    let #names: #types = args.get(stringify!(#names)).map(|arg| arg.parse().unwrap()).unwrap_or(#defaults);
                )*
                #block
            }
        }
    );
    quote! {
            #[no_mangle]
            pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
                #(
                    let #names: #types = args.get(stringify!(#names)).map(|arg| arg.parse().unwrap()).unwrap_or(#defaults);
                )*
                #block
            }
        }.into()
}

#[derive(Debug, Clone)]
struct Argument {
    pub attr: HelperAttribute,
    pub ident: proc_macro2::Ident,
    pub ty: proc_macro2::Ident,
}

impl Parse for Argument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr: HelperAttribute = input.parse()?;
        let ident: proc_macro2::Ident = input.parse()?;

        let _: syn::token::Colon = input.parse()?;
        let ty: proc_macro2::Ident = input.parse()?;
        println!("attr: {:?}, ident: {:?}, ty: {:?}\n", attr, ident, ty);
        Ok(Self { attr, ident, ty })
    }
}

#[derive(Debug, Clone)]
struct HelperAttribute {
    pub values: syn::punctuated::Punctuated<DefaultParam, syn::Token![,]>,
}

impl Parse for HelperAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr_content;
        let value_content;
        let _: syn::token::Pound = input.parse()?;
        bracketed!(attr_content in input);
        let ident: proc_macro2::Ident = attr_content.parse()?;
        if ident.to_string() != *"value" {
            return Err(syn::Error::new_spanned(
                ident.clone(),
                format!(
                    "Unknown attribute \"{}\" found, expected \"value\"",
                    ident
                ),
            ));
        }
        parenthesized!(value_content in attr_content);

        Ok(Self {
            values: syn::punctuated::Punctuated::parse_separated_nonempty_with(
                &value_content,
                DefaultParam::parse,
            )?,
        })
    }
}

#[derive(Debug, Clone)]
struct DefaultParam {
    pub ident: Option<proc_macro2::Ident>,
    pub val: syn::Expr,
}

impl Parse for DefaultParam {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let ident: Option<proc_macro2::Ident> = Some(input.parse()?);
            let _: syn::token::Eq = input.parse()?;
            Ok(Self {
                ident,
                val: input.parse()?,
            })
        } else {
            Ok(Self {
                ident: None,
                val: input.parse()?,
            })
        }
    }
}

#[derive(Debug)]
struct AttributeParameter {
    pub default: Option<syn::Expr>,
    pub min: Option<syn::Expr>,
    pub max: Option<syn::Expr>,
}

impl AttributeParameter {
    // TODO: Checking the Options is quite ugly
    pub fn new(
        default: Option<syn::Expr>,
        min: Option<syn::Expr>,
        max: Option<syn::Expr>,
    ) -> Self {
        if let Some(default) = default {
            let min = if min.is_some() { min } else { None };
            let max = if max.is_some() { max } else { None };
            Self {
                default: Some(default),
                min,
                max,
            }
        } else {
            let mut default = None;
            let max = if max.is_some() {
                default = max.clone();
                max
            } else {
                None
            };

            let min = if min.is_some() {
                default = min.clone();
                min
            } else {
                None
            };

            Self { default, min, max }
        }
    }
}

// #[fj::model]
// pub fn model(
//     #[default(5)] num_points: u64,
//     #[default(1.0)] r1: f64,
//     #[default(2.0)] r2: f64,
//     #[default(1.0)] h: f64,
// ) -> fj::Shape

// #[no_mangle]
// pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
//     let num_points: u64 = args
//         .get("num_points")
//         .map(|arg| arg.parse().unwrap())
//         .unwrap_or(5);

//     let r1: f64 = args
//         .get("r1")
//         .map(|arg| arg.parse().unwrap())
//         .unwrap_or(1.0);

//     let r2: f64 = args
//         .get("r2")
//         .map(|arg| arg.parse().unwrap())
//         .unwrap_or(2.0);

//     let h: f64 = args.get("h").map(|arg| arg.parse().unwrap()).unwrap_or(1.0);

// }
