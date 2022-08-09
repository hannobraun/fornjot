mod expand;
mod parse;

use proc_macro::TokenStream;
use syn::{parse_macro_input, FnArg, ItemFn};

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

    match parse::parse(&item) {
        Ok(init) => {
            let item = without_param_attrs(item);

            let tokens = quote::quote! {
                #item
                #init
            };

            tokens.into()
        }
        Err(e) => e.into_compile_error().into(),
    }
}

/// Strip out any of our `#[param(...)]` attributes so the item will compile.
fn without_param_attrs(mut item: ItemFn) -> ItemFn {
    for input in &mut item.sig.inputs {
        let attrs = match input {
            FnArg::Receiver(r) => &mut r.attrs,
            FnArg::Typed(t) => &mut t.attrs,
        };
        attrs.retain(|attr| !attr.path.is_ident("param"));
    }

    item
}
