use crate::codegen::rust::RustCodegen;
use crate::ir::expression::Expression;
use proc_macro2::TokenStream;
pub enum Statement {
    Expression(Expression),
}

impl RustCodegen for Statement {
    fn to_rust(&self) -> TokenStream {
        match self {
            Statement::Expression(e) => e.to_rust(),
        }
    }
}
