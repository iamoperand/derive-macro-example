use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// #[derive(MyTrait)]
// #[my_trait(answer = 0)]
// struct Bar;

#[proc_macro_derive(MyTrait)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl MyTrait for #ident {}
    };
    output.into()
}
