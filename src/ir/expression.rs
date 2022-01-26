use crate::codegen::rust::RustCodegen;
use crate::ir::method::MethodInvocation;
use proc_macro2::TokenStream;

pub enum Expression {
    MethodInvocation(MethodInvocation),
}

impl RustCodegen for Expression {
    fn to_rust(&self) -> TokenStream {
        match self {
            Expression::MethodInvocation(mi) => mi.to_rust(),
        }
    }
}
