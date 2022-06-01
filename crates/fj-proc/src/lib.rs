use proc_macro::TokenStream;

mod attributed_arguments;

#[proc_macro_attribute]
pub fn model(default_values: TokenStream, input: TokenStream) -> TokenStream {
    attributed_arguments::attributed_arguments(default_values, input)
}
