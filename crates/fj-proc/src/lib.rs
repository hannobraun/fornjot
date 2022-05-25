use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn model(default_values: TokenStream, input: TokenStream) -> TokenStream {
    let vals: Vec<String> = default_values
        .into_iter()
        .filter_map(|tree| {
            if let proc_macro::TokenTree::Literal(lit) = tree {
                Some(lit.to_string())
            } else {
                None
            }
        })
        .collect();
    let item = parse_macro_input!(input as syn::ItemFn);

    let inputs = item.sig.inputs;
    let mut names = Vec::new();
    let mut types = Vec::new();
    for f in inputs.iter() {
        if let syn::FnArg::Typed(meep) = f {
            if let syn::Pat::Ident(ident) = *meep.clone().pat {
                names.push(ident.ident);
            }
            if let syn::Type::Path(path) = *meep.clone().ty {
                types.push(path.path.get_ident().unwrap().clone());
            }
        }
    }
    let block = item.block;

    quote! {
            #[no_mangle]
            pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
                #(let #names: #types = args.get(stringify!(#names)).map(|arg| arg.parse().unwrap()).unwrap_or(#vals.parse::<#types>().unwrap());)*
                #block
            }
        }.into()
}

// #[fj_proc::model(5, 1.0, 2.0, 1.0)]
// pub fn model(num_points: u64, r1: f64, r2: f64, h: f64) -> fj::Shape {
// }

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
