use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::parse::{
    ArgumentMetadata, Constraint, ConstraintKind, ExtractedArgument,
    GeometryFunction, Initializer, Metadata, Model,
};

impl Initializer {
    fn register() -> TokenStream {
        quote! {
            const _: () = {
                fj::register_model!(|host| {
                    fj::models::HostExt::register_model(host, Model);

                    Ok(
                        fj::models::Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                            .with_short_description(env!("CARGO_PKG_DESCRIPTION"))
                            .with_homepage(env!("CARGO_PKG_HOMEPAGE"))
                            .with_repository(env!("CARGO_PKG_REPOSITORY"))
                            .with_license(env!("CARGO_PKG_LICENSE")),
                    )
                });
            };
        }
    }
}

impl ToTokens for Initializer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { model } = self;

        tokens.extend(Self::register());
        model.to_tokens(tokens);
    }
}

impl Model {
    fn definition() -> TokenStream {
        quote! { struct Model; }
    }

    fn trait_implementation(&self) -> TokenStream {
        let Self { metadata, geometry } = self;

        quote! {
            impl fj::models::Model for Model {
                #metadata
                #geometry
            }
        }
    }
}

impl ToTokens for Model {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(Self::definition());
        tokens.extend(self.trait_implementation());
    }
}

impl ToTokens for Metadata {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, arguments } = self;

        tokens.extend(quote! {
            fn metadata(&self) -> std::result::Result<fj::models::ModelMetadata, Box<dyn std::error::Error + Send + Sync +'static>> {
                Ok(fj::models::ModelMetadata::new(#name)
                #( .with_argument(#arguments) )*)
            }
        });
    }
}

impl ToTokens for ArgumentMetadata {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            default_value,
        } = self;

        tokens.extend(quote! { fj::models::ArgumentMetadata::new(#name) });

        if let Some(default_value) = default_value {
            tokens.extend(quote! {
                .with_default_value(stringify!(#default_value))
            });
        }
    }
}

impl ToTokens for GeometryFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            geometry_function,
            arguments,
            constraints,
            fallible,
        } = self;

        let argument_names = arguments.iter().map(|a| &a.ident);

        let invocation = quote! {
            #geometry_function(#( #argument_names ),*)
        };
        let invocation = if *fallible {
            quote! { #invocation.map(fj::Shape::from).map_err(Into::into) }
        } else {
            quote! { Ok(#invocation.into()) }
        };

        tokens.extend(quote! {
            fn shape(
                &self,
                ctx: &dyn fj::models::Context,
            ) -> Result<fj::Shape, fj::models::Error> {
                #( #arguments )*
                #( #constraints )*
                #invocation
            }
        });
    }
}

impl ToTokens for ExtractedArgument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident,
            ty,
            default_value,
        } = self;

        let name = ident.to_string();
        let t = match default_value {
            Some(default) => quote! {
                let #ident: #ty = match ctx.get_argument(#name) {
                    Some(value) => value.parse()?,
                    None => #default
                };
            },
            None => {
                let error_message = format!("Expected {name}");
                quote! {
                    let #ident: #ty = match ctx.get_argument(#name) {
                        Some(value) => value.parse()?,
                        None => return Err(#error_message.into()),
                    };
                }
            }
        };

        tokens.extend(t);
    }
}

impl ToTokens for Constraint {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { target, expr, kind } = self;

        let operator = match kind {
            ConstraintKind::Max => quote!(<=),
            ConstraintKind::Min => quote!(>=),
        };
        let predicate = quote! { #target #operator #expr };
        // Note: this will cause `expr` to be evaluated twice. Predicates should
        // be pure functions, so in theory this shouldn't be an issue.
        let error_message = quote! {
            format!(
                "Expected {} {} {} (i.e. {} {} {})",
                stringify!(#target),
                stringify!(#operator),
                stringify!(#expr),
                #target,
                stringify!(#operator),
                #expr,
            )
        };

        tokens.extend(quote! {
            if !(#predicate) {
                return Err(#error_message.into());
            }
        });
    }
}
