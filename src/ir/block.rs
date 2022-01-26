use crate::codegen::rust::RustCodegen;
use crate::ir::statement::Statement;
use proc_macro2::TokenStream;
use quote::quote;

pub type Block = Vec<Statement>;

impl RustCodegen for Block {
    fn to_rust(&self) -> TokenStream {
        let statements: Vec<_> = self.iter().map(RustCodegen::to_rust).collect();
        quote! {
            {
                #(#statements)*
            }
        }
    }
}
