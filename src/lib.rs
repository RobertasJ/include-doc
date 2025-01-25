use std::fs::read_to_string;

use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro_attribute]
pub fn include_doc(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let path = parse_macro_input!(attr as LitStr);
    let path = path.value();

    let md = read_to_string(&path).unwrap_or_else(|_| panic!("Could not find {path:?}"));
    let md = format!("\n\n{}\n\n\n", md.trim());

    quote! {
        #[doc = #md]
        #input
    }
    .into()
}
