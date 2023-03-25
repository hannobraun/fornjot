use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::parse::{
    ArgumentMetadata, Constraint, ConstraintKind, ExtractedArgument,
    GeometryFunction, Initializer, Metadata, Model,
};

impl Initializer {
    fn register(&self) -> TokenStream {
        let model_name = format_ident!("{}", self.model.metadata.name);

        // pub(crate) ident: Ident,
        // pub(crate) ty: Type,
        // pub(crate) default_value: Option<Expr>,

        let arguments = self.model.geometry.arguments.iter().map(|argument| {
            let name = argument.ident.to_string();

            let default_value = &argument.default_value;

            if let Some(default_value) = default_value {
                quote::quote! { arguments.get(#name).map(|argument| argument.unpack()).unwrap_or(#default_value) }
            } else {
                let error_message = format!(
                    "Host did not provide value for parameter \"{}\".",
                    name
                );
                quote::quote! { arguments.get(#name).expect(#error_message).unpack() }
            }
        });

        quote! {
            const _: () = {
                #[no_mangle]
                unsafe extern "C" fn fj_model_construct(
                    payload_pointer: *mut *mut u8,
                    arguments_pointer: *const u8,
                    arguments_length: usize,
                ) -> usize {
                    use fj::abi::{SelfSerializing, UnpackParameter};

                    fj::abi::initialize_panic_handling();

                    let arguments = std::slice::from_raw_parts(arguments_pointer, arguments_length);
                    let arguments = fj::abi::ArgumentTable::deserialize(arguments);

                    let model_result = match arguments {
                        Ok(arguments) => {
                            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                #model_name(#(#arguments),*)
                            })) {
                                Ok(shape) => fj::abi::ModelResult::Ok(
                                    fj::abi::Model {
                                        metadata: fj::models::Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                                            .with_short_description(env!("CARGO_PKG_DESCRIPTION"))
                                            .with_homepage(env!("CARGO_PKG_HOMEPAGE"))
                                            .with_repository(env!("CARGO_PKG_REPOSITORY"))
                                            .with_license(env!("CARGO_PKG_LICENSE")),
                                        shape,
                                    }
                                ),
                                Err(payload) => {
                                    fj::abi::ModelResult::Panic(fj::abi::on_panic(payload))
                                }
                            }
                        },
                        Err(error) => {
                            fj::abi::ModelResult::Error("Failed to deserialize parameters from host.".to_string())
                        }
                    };

                    match model_result.serialize() {
                        Ok(model_result) => {
                            let length = model_result.len();
                            let boxed = model_result.into_boxed_slice();

                            *payload_pointer = Box::into_raw(boxed) as *mut u8;
                            length
                        },
                        Err(_error) => {
                            *payload_pointer = std::ptr::null_mut();
                            0
                        }
                    }
                }

                #[no_mangle]
                unsafe extern "C" fn fj_model_free(ptr: *mut u8) {
                    // We just take it back and then immediately drop it.
                    let _ = Box::from_raw(ptr);
                }
            };
        }
    }
}

impl ToTokens for Initializer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.register());
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
