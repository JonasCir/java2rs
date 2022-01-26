use proc_macro2::TokenStream;

pub trait RustCodegen {
    fn to_rust(&self) -> TokenStream;
}
