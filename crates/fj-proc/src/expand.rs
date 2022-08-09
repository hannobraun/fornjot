use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::parse::{
    ArgumentMetadata, Constraint, ConstraintKind, ExtractedArgument,
    GeometryFunction, Initializer, Metadata, Model,
};

impl Initializer {
    fn register(&self) -> TokenStream {
        quote! {
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
        }
    }
}

impl ToTokens for Initializer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Initializer { model } = self;

        let register = self.register();

        tokens.extend(quote! {
            const _: () = {
                use fj::models::internal::*;
                #register
                #model
            };
        });
    }
}

impl Model {
    fn definition(&self) -> TokenStream {
        quote! { struct Model; }
    }

    fn trait_implementation(&self) -> TokenStream {
        let Model { metadata, geometry } = self;

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
        tokens.extend(self.definition());
        tokens.extend(self.trait_implementation());
    }
}

impl ToTokens for Metadata {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Metadata { name, arguments } = self;

        tokens.extend(quote! {
            fn metadata(&self) -> fj::models::ModelMetadata {
                fj::models::ModelMetadata::new(#name)
                #( .with_argument(#arguments) )*
            }
        });
    }
}

impl ToTokens for ArgumentMetadata {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ArgumentMetadata {
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
        let GeometryFunction {
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
            quote! { ROk(#invocation.into()) }
        };

        tokens.extend(quote! {
            fn shape(
                &self,
                ctx: fj::models::internal::Context<'_>,
            ) -> fj::models::internal::RResult<fj::Shape, fj::models::Error> {
                #( #arguments )*
                #( #constraints )*
                #invocation
            }
        });
    }
}

impl ToTokens for ExtractedArgument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ExtractedArgument {
            ident,
            ty,
            default_value,
        } = self;

        let name = ident.to_string();
        let t = match default_value {
            Some(default) => quote! {
                let #ident: #ty = match ctx.get_argument(#name.into()) {
                    RSome(value) => match value.parse() {
                        Ok(v) => v,
                        Err(e) => return RErr(RBoxError::new(e)),
                    },
                    RNone => #default
                };
            },
            None => {
                let error_message = format!("Expected {name}");
                quote! {
                    let #ident: #ty = match ctx.get_argument(#name.into()) {
                        RSome(value) => match value.parse() {
                            Ok(v) => v,
                            Err(e) => return RErr(RBoxError::new(e)),
                        },
                        RNone => return RErr(#error_message.into()),
                    };
                }
            }
        };

        tokens.extend(t);
    }
}

impl ToTokens for Constraint {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Constraint { target, expr, kind } = self;

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
                return RErr(RBoxError::from_fmt(&#error_message));
            }
        });
    }
}
